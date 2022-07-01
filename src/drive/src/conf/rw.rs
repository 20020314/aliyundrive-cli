use serde::ser;

pub trait RW<T1, T2>
where
    T1: ser::Serialize,
    T2: ser::Serialize,
{
    fn print_std();

    fn write(t: T1) -> anyhow::Result<()>;

    fn read() -> anyhow::Result<T1>;

    fn read_token(is_mobile: bool) -> anyhow::Result<T2>;

    fn write_token(is_mobile: bool, t: T2) -> anyhow::Result<()>;
}
