/*
 * @Date: 2021-10-01 01:22:51
 * @LastEditors: LIULIJING
 * @LastEditTime: 2021-10-01 01:24:22
 */
fn main() {
    prost_build::Config::new()
        .out_dir("src/pb")
        .compile_protos(&["abi.proto"], &["."])
        .unwrap()
}