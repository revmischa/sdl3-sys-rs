#![allow(unused_macros)]

#[cfg(not(feature = "const_trait_impl"))]
macro_rules! make_bit_ops_impls_for {
  ($ty_name:ty) => {
    impl ::core::ops::BitAnd for $ty_name {
      type Output = Self;
      #[inline]
      #[must_use]
      fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
      }
    }
    impl ::core::ops::BitOr for $ty_name {
      type Output = Self;
      #[inline]
      #[must_use]
      fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
      }
    }
    impl ::core::ops::BitXor for $ty_name {
      type Output = Self;
      #[inline]
      #[must_use]
      fn bitxor(self, rhs: Self) -> Self {
        Self(self.0 ^ rhs.0)
      }
    }
    impl ::core::ops::Not for $ty_name {
      type Output = Self;
      #[inline]
      #[must_use]
      fn not(self) -> Self {
        Self(!self.0)
      }
    }
    impl ::core::ops::BitAndAssign for $ty_name {
      #[inline]
      fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0);
      }
    }
    impl ::core::ops::BitOrAssign for $ty_name {
      #[inline]
      fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0);
      }
    }
    impl ::core::ops::BitXorAssign for $ty_name {
      #[inline]
      fn bitxor_assign(&mut self, rhs: Self) {
        self.0.bitxor_assign(rhs.0);
      }
    }
  };
}

#[cfg(feature = "const_trait_impl")]
macro_rules! make_bit_ops_impls_for {
  ($ty_name:ty) => {
    impl const ::core::ops::BitAnd for $ty_name {
      type Output = Self;
      #[inline]
      #[must_use]
      fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
      }
    }
    impl const ::core::ops::BitOr for $ty_name {
      type Output = Self;
      #[inline]
      #[must_use]
      fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
      }
    }
    impl const ::core::ops::BitXor for $ty_name {
      type Output = Self;
      #[inline]
      #[must_use]
      fn bitxor(self, rhs: Self) -> Self {
        Self(self.0 ^ rhs.0)
      }
    }
    impl const ::core::ops::Not for $ty_name {
      type Output = Self;
      #[inline]
      #[must_use]
      fn not(self) -> Self {
        Self(!self.0)
      }
    }
    impl ::core::ops::BitAndAssign for $ty_name {
      #[inline]
      fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0);
      }
    }
    impl ::core::ops::BitOrAssign for $ty_name {
      #[inline]
      fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0);
      }
    }
    impl ::core::ops::BitXorAssign for $ty_name {
      #[inline]
      fn bitxor_assign(&mut self, rhs: Self) {
        self.0.bitxor_assign(rhs.0);
      }
    }
  };
}
