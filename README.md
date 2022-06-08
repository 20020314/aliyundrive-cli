# aliyundrive-cli

> 阿里云盘`Terminal CLI`工具，支持`Terminal`终端`QR Code`扫码登录

### Example
> 扫码登陆example，返回Web端和移动端Token
- Web端token适用[aliyundrive-webdav](https://github.com/messense/aliyundrive-webdav)、[aliyundrive-fuse](https://github.com/messense/aliyundrive-fuse)
- 移动端token适用于[alist](https://github.com/Xhofe/alist)、最新[PR](https://github.com/messense/aliyundrive-webdav/pull/445)支持aliyundrive-webdav
```shell
 $ git clone https://github.com/gngpp/aliyundrive-cli.git && cd aliyundrive-cli
 $ cargo run --color=always --example token --manifest-path ./drive/Cargo.toml   
```
