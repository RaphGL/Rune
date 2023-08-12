pub enum TVSystem {
    NTSC,
    PAL,
    DUAL,
}

/// The first 16 bytes of a INES file
#[repr(C)]
#[derive(Debug)]
pub struct InesHeader {
    /// should contain [0x4E, 0x45, 0x53, 0x1A]
    constant: [u8; 4],
    /// PRG ROM data size: 16384 * x bytes
    pub prg_size: u8,
    /// CHR ROM data size: 8192 * y bytes
    /// chr_size = 0 means the board uses CHR RAM
    pub chr_size: u8,
    /// mapper, mirroring, battery, trainer
    flags6: u8,
    /// mapper, vs/playchoice, nes 2.0
    flags7: u8,
    /// prg_ram size
    flags8: u8,
    /// tv system
    flags9: u8,
    /// tv system, prg_ram presence
    flags10: u8,
    /// should be filled with zeros
    padding: [u8; 5],
}

impl InesHeader {
    pub fn parse(header: &[u8]) -> Result<InesHeader, InesHeader> {
        let header = InesHeader {
            constant: header[0..=3]
                .try_into()
                .expect("could not convert slice to array"),
            prg_size: header[4],
            chr_size: header[5],
            flags6: header[6],
            flags7: header[7],
            flags8: header[8],
            flags9: header[9],
            flags10: header[10],
            padding: header[11..16]
                .try_into()
                .expect("could not convert slice to array"),
        };

        if header.constant[0] == 0x4e
            && header.constant[1] == 0x45
            && header.constant[2] == 0x53
            && header.constant[3] == 0x1a
        {
            return Ok(header);
        }

        Err(header)
    }

    // flags 6
    pub fn has_vertical_arrangement(&self) -> bool {
        self.flags6 & 0b0000_0001 == 1
    }

    pub fn has_horizontal_arrangement(&self) -> bool {
        self.flags6 & 0b0000_0001 == 0
    }

    pub fn has_persistent_memory(&self) -> bool {
        self.flags6 & 0b0000_0010 == 0b10
    }

    pub fn has_trainer(&self) -> bool {
        self.flags6 & 0b0000_0100 == 0b100
    }

    pub fn ignores_mirroring_ctl(&self) -> bool {
        self.flags6 & 0b0000_1000 == 0b1000
    }

    fn get_lower_mapper_nibble(&self) -> u8 {
        (self.flags6 & 0xf0) >> 4
    }

    // flags 7
    pub fn is_vs_unisystem(&self) -> bool {
        self.flags7 & 0b0000_0001 == 1
    }

    pub fn is_playchoice10(&self) -> bool {
        self.flags7 & 0b0000_0010 == 0b10
    }

    pub fn is_nes20(&self) -> bool {
        self.flags7 & 0b0000_1100 == 0b1000
    }

    fn get_upper_mapper_nibble(&self) -> u8 {
        self.flags7 & 0b1111_0000
    }

    // flags 8
    pub fn get_prg_ram_size(&self) -> u8 {
        self.flags8
    }

    // flags 9
    // no roms in circulation use this bit thus it is ignored

    // flags 10
    /// not part of the official spec, so it should not be mandatory
    pub fn get_tv_system(&self) -> TVSystem {
        let system = self.flags10 & 0b0000_0011;

        match system {
            0 => TVSystem::NTSC,
            2 => TVSystem::PAL,
            1 | 3 => TVSystem::DUAL,
            _ => unreachable!(),
        }
    }

    pub fn has_prg_ram(&self) -> bool {
        self.flags10 & 0b0001_0000 == 0b0001_0000
    }

    pub fn has_board_conflicts(&self) -> bool {
        self.flags10 & 0b0010_0000 == 0b0010_0000
    }

    /// returns the number of the mapper used by the ROM 
    pub fn get_mapper(&self) -> u8 {
        self.get_upper_mapper_nibble() | self.get_lower_mapper_nibble()
    }
}

#[derive(Debug)]
pub struct InesFile {
    pub header: InesHeader,
    pub trainer: Option<Vec<u8>>,
    pub prg_rom: Vec<u8>,
    pub chr_rom: Vec<u8>,
    pub inst_rom: Option<Vec<u8>>,
    pub prom: Option<Vec<u8>>,
}

impl InesFile {
    /// returns an InesFile loaded with contents of file
    pub fn open(filename: &str) -> InesFile {
        let file = std::fs::read(filename).unwrap();
        let header = InesHeader::parse(&file[0..16]).unwrap();

        let mut curr = 16;

        let trainer = match header.has_trainer() {
            true => {
                let trainer = Some(file[curr..(curr + 512)].to_vec());
                curr += 512;
                trainer
            }
            false => None,
        };

        let prg_rom_end = curr + header.prg_size as usize * 16384;
        let prg_rom = file[curr..prg_rom_end].to_vec();
        curr = prg_rom_end;

        let chr_rom_end = curr + header.chr_size as usize * 8192;
        let chr_rom = file[curr..chr_rom_end].to_vec();
        curr = chr_rom_end;

        let inst_rom = match header.is_playchoice10() {
            true => Some(file[curr..(curr + 8192)].to_vec()),
            false => None,
        };

        InesFile {
            header,
            trainer,
            prg_rom,
            chr_rom,
            inst_rom,
            prom: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_non_nes_files() {
        let mut header = [0u8; 16];
        assert!(InesHeader::parse(&header).is_err());

        header[0] = 0x4e;
        header[1] = 0x45;
        header[2] = 0x53;
        header[3] = 0x1a;
        assert!(InesHeader::parse(&header).is_ok());
    }

    #[test]
    fn flags6() {
        let mut header: InesHeader = unsafe { std::mem::zeroed() };
        header.flags6 = 1;
        assert!(header.has_vertical_arrangement());
        header.flags6 = 0;
        assert!(header.has_horizontal_arrangement());

        header.flags6 = 0b0000_0010;
        assert!(header.has_persistent_memory());
        header.flags6 = 0;
        assert!(!header.has_persistent_memory());

        header.flags6 = 0b0000_0100;
        assert!(header.has_trainer());
        header.flags6 = 0;
        assert!(!header.has_trainer());

        header.flags6 = 0b0000_1000;
        assert!(header.ignores_mirroring_ctl());
        header.flags6 = 0;
        assert!(!header.ignores_mirroring_ctl());

        header.flags6 = 0b1111_0000;
        assert!(header.get_lower_mapper_nibble() == 0b1111);
        header.flags6 = 0;
        assert!(header.get_lower_mapper_nibble() == 0);
    }

    #[test]
    fn flags7() {
        let mut header: InesHeader = unsafe { std::mem::zeroed() };
        header.flags7 = 1;
        assert!(header.is_vs_unisystem());
        header.flags7 = 0;
        assert!(!header.is_vs_unisystem());

        header.flags7 = 0b0000_0010;
        assert!(header.is_playchoice10());
        header.flags7 = 0;
        assert!(!header.is_playchoice10());

        header.flags7 = 0b0000_1000;
        assert!(header.is_nes20());
        header.flags7 = 0;
        assert!(!header.is_nes20());

        header.flags7 = 0b1111_0000;
        assert!(header.get_upper_mapper_nibble() == 0b1111_0000);
        header.flags7 = 0;
        assert!(header.get_upper_mapper_nibble() == 0);
    }

    #[test]
    fn flags8() {
        let mut header: InesHeader = unsafe { std::mem::zeroed() };
        header.flags8 = 243;
        assert!(header.get_prg_ram_size() == 243);
    }

    #[test]
    fn flags9() {
        // not used by any ROMs in circulation thus it is ignored
    }

    #[test]
    fn flags10() {
        let mut header: InesHeader = unsafe { std::mem::zeroed() };
        header.flags10 = 0;
        assert!(match header.get_tv_system() {
            TVSystem::NTSC => true,
            _ => false,
        });

        header.flags10 = 1;
        assert!(match header.get_tv_system() {
            TVSystem::DUAL => true,
            _ => false,
        });
        header.flags10 = 3;
        assert!(match header.get_tv_system() {
            TVSystem::DUAL => true,
            _ => false,
        });

        header.flags10 = 2;
        assert!(match header.get_tv_system() {
            TVSystem::PAL => true,
            _ => false,
        });

        header.flags10 = 0b0001_0000;
        assert!(header.has_prg_ram());
        header.flags10 = 0;
        assert!(!header.has_prg_ram());

        header.flags10 = 0b0010_0000;
        assert!(header.has_board_conflicts());
        header.flags10 = 0;
        assert!(!header.has_board_conflicts());
    }
}
