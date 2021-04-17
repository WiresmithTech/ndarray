
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

    /// Return Order::RowMajor if the input is true, Order::ColumnMajor otherwise
    #[inline]
    pub fn use_c(use_c: bool) -> Order {
        if use_c { Order::C } else { Order::F }
    }

    /// Return Order::ColumnMajor if the input is true, Order::RowMajor otherwise
    #[inline]
    pub fn use_f(use_f: bool) -> Order {
        Self::use_c(!use_f)
    }

    /// Return the transpose: row major becomes column major and vice versa.
    /// Note that automatic stays automatic.
    #[inline]
    pub fn transpose(self) -> Order {
        match self {
            Order::RowMajor => Order::ColumnMajor,
            Order::ColumnMajor => Order::RowMajor,
            Order::Automatic => Order::Automatic,
        }
    }
}
