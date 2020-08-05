/*
 * This file is part of the coreboot project.
 *
 * Copyright (C) 2020 Google Inc.
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; version 2 of the License.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 */

pub struct MainBoard {
}

impl MainBoard {
    pub fn new(base: usize, baudrate: u32) -> MainBoard {
        MainBoard { }
    }
}

impl Driver for MainBoard {
    fn init(&mut self) -> Result<()> {
        Ok(())
    }

    fn pread(&self, data: &mut [u8], _offset: usize) -> Result<usize> {
            return Ok(0);
    }

    fn pwrite(&mut self, data: &[u8], _offset: usize) -> Result<usize> {
        Ok(data.len())
    }

    fn shutdown(&mut self) {}
}

impl ClockNode for MainBoard {
    // This uses hfclk as the input rate.
    fn set_clock_rate(&mut self, rate: u32) {
    }
}

