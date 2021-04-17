
/// Array order description
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum Order {
    /// Row major or "C" order
    RowMajor,
    /// Column major or "F" order
    ColumnMajor,
    /// Try to preserve the input order
    Automatic,
}

impl Order {
    /// "C" is an alias for row major ordering
    pub const C: Order = Order::RowMajor;

    /// "F" (for Fortran) is an alias for column major ordering
    pub const F: Order = Order::ColumnMajor;

    /// "A"  is an alias for automatic ordering
    pub const A: Order = Order::Automatic;
}
