// Copyright 2022 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use crate::util::write_value;
use concat_in_place::strcat;
use std::fs;

pub fn powersave() {
    if let Some((cpus, (min, mut max))) =
        num_cpus().zip(frequency_minimum().zip(frequency_maximum()))
    {
        max /= 2;
        for cpu in 0..cpus {
            set_frequency_minimum(cpu, min);
            set_frequency_maximum(cpu, max);
            set_governor(cpu, "powersave\n");
        }
    }
}

pub fn performance() {
    if let Some((cpus, (min, max))) = num_cpus().zip(frequency_minimum().zip(frequency_maximum())) {
        for cpu in 0..cpus {
            set_frequency_minimum(cpu, min);
            set_frequency_maximum(cpu, max);
            set_governor(cpu, "performance\n");
        }
    }
}

pub fn num_cpus() -> Option<usize> {
    let info = fs::read_to_string("/sys/devices/system/cpu/possible").ok()?;
    info.split('-').nth(1)?.trim_end().parse::<usize>().ok()
}

pub fn frequency_maximum() -> Option<usize> {
    let mut sys_path = sys_path(0);
    let path = strcat!(&mut sys_path, "cpuinfo_max_freq");
    let string = fs::read_to_string(path).ok()?;
    string.trim_end().parse::<usize>().ok()
}

pub fn frequency_minimum() -> Option<usize> {
    let mut sys_path = sys_path(0);
    let path = strcat!(&mut sys_path, "cpuinfo_min_freq");
    let string = fs::read_to_string(path).ok()?;
    string.trim_end().parse::<usize>().ok()
}

pub fn set_frequency_maximum(core: usize, frequency: usize) {
    let mut sys_path = sys_path(core);
    let path = strcat!(&mut sys_path, "scaling_max_freq");
    write_value(path, frequency);
}

pub fn set_frequency_minimum(core: usize, frequency: usize) {
    let mut sys_path = sys_path(core);
    let path = strcat!(&mut sys_path, "scaling_min_freq");
    write_value(path, frequency);
}

pub fn set_governor(core: usize, governor: &str) {
    let mut sys_path = sys_path(core);
    let path = strcat!(&mut sys_path, "scaling_governor");
    write_value(path, governor);
}

fn sys_path(core: usize) -> String { format!("/sys/devices/system/cpu/cpu{}/cpufreq/", core) }