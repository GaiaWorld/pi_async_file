#![feature(once_cell)]
#![feature(io_error_more)]
//! # 异步文件系统
//!

#![allow(warnings)]
extern crate parking_lot;
extern crate pi_async_rt;
extern crate crossbeam_utils;
extern crate sysinfo;
extern crate normpath;

pub mod file;