pub enum Extensions{
    Zip,
    Rar,
    Tar,
    Bz2,
    Tbz2,
    Tgz,
    Txz,
    Lzma,
    Gz,
    Xz,
    Z,
    Sevenz,
    Arj,
    Cab,
    Chm,
    Deb,
    Dmg,
    Iso,
    Lzh,
    Msi,
    Rpm,
    Udf,
    Wim,
    Xar,
    Exe  
};

pub struct File{
    filename: String;
    extensions: <Extensions>;
    path: String;
};

pub struct Zip{};
pub struct Rar{};
pub struct Tar{};
pub struct Bz2{};
pub struct Tbz2{};
pub struct Tgz{};
pub struct Txz{};
pub struct Lzma{};
pub struct Gz{};
pub struct Gz{};
pub struct Xz{};
pub struct Z{};
pub struct Sevenz{};
pub struct Arj{};
pub struct Cab{};
pub struct Chm{};
pub struct Deb{};
pub struct Dmg{};
pub struct Iso{};
pub struct Lzh{};
pub struct Msi{};
pub struct Rpm{};
pub struct Udf{};
pub struct Wim{};
pub struct Xar{};
pub struct Exe{}; 
