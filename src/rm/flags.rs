bitflags! {
    /// Flag values that are used by the methods in
    /// [`ResourceManager`](trait.ResourceManager.html).
    #[derive(Default)]
    pub struct Flags: u32 {
        // /  No resource manager feature selected.
        // const NO_FLAGS = 0;

        /// For [`ResourceManager::xa_start()`](trait.ResourceManager.html#method.xa_start):
        /// indicates that the resource should associate with a previously suspended transaction.
        const RESUME = 0x08_00_00_00;

        /// For [`ResourceManager::xa_start()`](trait.ResourceManager.html#method.xa_start):
        /// indicates that the transaction should associate with a transaction
        /// previously seen by the server.
        const JOIN = 0x00_20_00_00;

        /// For [`ResourceManager::xa_recover()`](trait.ResourceManager.html#method.xa_recover):
        /// indicates that the server should start a new recovery scan.
        const START_RECOVERY_SCAN = 0x01_00_00_00;

        /// For [`ResourceManager::xa_recover()`](trait.ResourceManager.html#method.xa_recover):
        /// indicates that the server should end the current recovery scan.
        const END_RECOVERY_SCAN = 0x00_80_00_00;

        /// Indicates that the caller is using one-phase optimization. Seems not to be used.
        const ONE_PHASE = 0x40_00_00_00;

        /// For [`ResourceManager::xa_end()`](trait.ResourceManager.html#method.xa_end):
        /// indicates that the transaction should be disassociated,
        /// and that the work has failed
        const FAIL = 0x20_00_00_00;

        /// For [`ResourceManager::xa_end()`](trait.ResourceManager.html#method.xa_end):
        /// indicates that the transaction should be disassociated,
        /// and that the work has completed sucessfully.
        const SUCCESS = 0x04_00_00_00;

        /// For [`ResourceManager::xa_end()`](trait.ResourceManager.html#method.xa_end):
        /// indicates that the resource should temporarily suspend the association
        /// with the transaction.
        const SUSPEND = 0x02_00_00_00;
    }
}
impl Flags {
    /// Returns `true` if only the flags in `other` are contained within `self`.
    #[inline]
    pub fn contains_only(&self, other: Flags) -> bool {
        (*self & !other).is_empty()
    }
}

#[cfg(test)]
mod test {
    use super::Flags;

    #[test]
    fn test1() {
        assert_eq!(
            Flags::RESUME | Flags::JOIN,
            Flags::from_bits(0x08_20_00_00).unwrap()
        );
        assert_eq!((Flags::RESUME | Flags::JOIN).bits(), 0x08_20_00_00);
        assert_eq!(Flags::default().bits(), 0);

        let pattern = Flags::FAIL | Flags::SUCCESS | Flags::SUSPEND;
        assert!((Flags::FAIL | Flags::SUCCESS).contains_only(pattern));
        assert!(Flags::SUCCESS.contains_only(pattern));
        assert!(Flags::default().contains_only(pattern));
        assert!(!Flags::RESUME.contains_only(pattern));
    }
}
