use std::{collections::BTreeMap, rc::Weak, sync::{Arc, RwLock}};
use std::time::SystemTime;
use spinning_top::Spinlock;
///定义inode名称的最大长度和块的最大值
pub const RAMS_MAX_NAMELEN:usize=64;
pub const RAMS_BLOCK_SIZE:u64=512;
///@brief 内存文件系统的Inode结构体，
/// 使用SpinLock自旋锁防止阻塞
#[derive(Debug)]
struct LockedRamFSInode(SpinLock<RamFSInode>);

#[derive(Debug,Clone)]
// pub struct Magic(u64);
// impl Magic {
//     pub const RAMFS_MAGIC :Magic  = Magic(0x72616D66); 
//     // 其他文件系统的魔数可以在这里定义
// }
// impl Magic{
//     pub get_magic()
// }
enum Magic{
    RAMFS_MAGIC=0x72616D66,
}
///@brief SuperBlock定义
#[derive(Debug, Clone)]
pub struct SuperBlock {
    /// 文件系统的块大小
    block_size: u64,
    ///文件名字的最大大小
    max_namelen:u64,
    ///文件标识Magic
    magic:Magic,
    // /// 文件系统中的总块数
    // total_blocks: u64,
    // /// 文件系统中的空闲块数
    // free_blocks: u64,
    // /// 文件系统中的总inode数
    // total_inodes: u64,
    // /// 文件系统中的空闲inode数
    // free_inodes: u64,
    // /// 文件系统的创建时间
    // creation_time: SystemTime,
    // /// 文件系统的修改时间
    // modification_time: SystemTime,
    // /// 文件系统的挂载次数
    // mount_count: u32,
    // /// 文件系统的最大挂载次数
    // max_mount_count: u32,
    // /// 文件系统的UUID
    // uuid: [u8; 16],
    // /// 文件系统的卷标
    // volume_label: String,
}

impl SuperBlock {
    pub fn new(magic:Magic,block_size:u64,max_namelen:u64)-> Arc<Self>{
        let block_size=block_size;
        let max_namelen=max_namelen;
        let magic=magic;

        let result: Arc<SuperBlock>=Arc::new(SuperBlock{
            block_size:block_size,
            max_namelen:max_namelen,
            magic:magic,
        });
        return result;
    }
}

///@brief 内存文件系统结构体,
/// 包含根Inode和超级块
#[derive(Debug)]
pub struct RamFS{
    //使用Arc保证数据安全,RwLock保证读的可以有多个，写的只能有一个
    root_inode: Arc<LockedRamFSInode>,
    super_block: RwLock<SuperBlock>,
}
///DName定义
#[derive(Debug)]
pub struct DName(String);

///iNode的元数据
#[derive(Debug, Clone)]
pub struct Metadata {
    /// 文件的大小（以字节为单位）
    size: u64,
    /// 文件的权限
    permissions: Permissions,
    /// 文件的创建时间
    creation_time: SystemTime,
    /// 文件的最后修改时间
    modification_time: SystemTime,
    /// 文件的最后访问时间
    access_time: SystemTime,
    /// 文件类型（如普通文件、目录、符号链接等）
    file_type: FileType,
}

///@brief 文件权限
#[derive(Debug, Clone)]
pub struct Permissions {
    /// 是否为只读
    readonly: bool,
    /// 用户权限
    user: u32,
    /// 组权限
    group: u32,
    /// 其他用户权限
    others: u32,
}

///@brief 文件类型
#[derive(Debug, Clone)]
pub enum FileType {
    File,
    Directory,
    Symlink,
    Other,
}

///@brief Inode结构体
#[derive(Debug)]
pub struct RamFSInode{
    name: DName,
    parent: Weak<LockedRamFSInode>,
    self_ref: Weak<LockedRamFSInode>,
    children: BTreeMap<DName,Arc<LockedRamFSInode>>,
    data: Vec<u8>,
    metadata:Metadata,
    fs:Weak<RamFS>,//用于获取文件系统信息，比如说superBlock结构体存储的信息
}

impl RamFS {
    pub fn new()->Arc<Self>{
        let super_block = SuperBlock::new(
            Magic::RAMFS_MAGIC,
            RAMFS_BLOCK_SIZE,
            RAMFS_MAX_NAMELEN as u64,
        );
    }
}











fn main(){

}

