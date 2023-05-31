enum TVSystem {
    NTSC,
    PAL,
    DUAL,
}

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
    fn parse(header: &[u8]) -> Result<InesHeader, InesHeader> {
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
    fn has_vertical_arrangement(&self) -> bool {
        self.flags6 & 0b0000_0001 == 1
    }

    fn has_horizontal_arrangement(&self) -> bool {
        self.flags6 & 0b0000_0001 == 0
    }

    fn has_persistent_memory(&self) -> bool {
        self.flags6 & 0b0000_0010 == 0b10
    }

    fn has_trainer(&self) -> bool {
        self.flags6 & 0b0000_0100 == 0b100
    }

    fn ignores_mirroring_ctl(&self) -> bool {
        self.flags6 & 0b0000_1000 == 0b1000
    }

    fn get_lower_mapper_nibble(&self) -> u8 {
        (self.flags6 & 0xf0) >> 4
    }

    // flags 7
    fn is_vs_unisystem(&self) -> bool {
        self.flags7 & 0b0000_0001 == 1
    }

    fn is_playchoice10(&self) -> bool {
        self.flags7 & 0b0000_0010 == 0b10
    }

    fn is_nes20(&self) -> bool {
        self.flags7 & 0b0000_1100 == 0b1000
    }

    fn get_higher_mapper_nibble(&self) -> u8 {
        self.flags7 & 0b1111_0000
    }

    // flags 8
    fn get_prg_ram_size(&self) -> u8 {
        self.flags8
    }

    // flags 9
    // no roms in circulation use this bit thus it is ignored

    // flags 10
    /// not part of the official spec, so it should not be mandatory
    fn get_tv_system(&self) -> TVSystem {
        let system = self.flags10 & 0b0000_0011;

        match system {
            0 => TVSystem::NTSC,
            2 => TVSystem::PAL,
            1 | 3 => TVSystem::DUAL,
            _ => unreachable!(),
        }
    }

    fn has_prg_ram(&self) -> bool {
        self.flags10 & 0b0001_0000 == 0b0001_0000
    }

    fn has_board_conflicts(&self) -> bool {
        self.flags10 & 0b0010_0000 == 0b0010_0000
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
    pub fn new(filename: &str) -> InesFile {
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

}
