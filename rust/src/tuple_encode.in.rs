impl<T0: Encode, > Encode for (T0, ) {
    fn encode(&self, out: &mut impl io::Write) -> Result {
        out.write_u8(104)?;
        out.write_u8(1)?;
        self.0.encode(out)?;
        Ok(())
    }
}

impl<T0: Encode, T1: Encode, > Encode for (T0, T1, ) {
    fn encode(&self, out: &mut impl io::Write) -> Result {
        out.write_u8(104)?;
        out.write_u8(2)?;
        self.0.encode(out)?;
        self.1.encode(out)?;
        Ok(())
    }
}

impl<T0: Encode, T1: Encode, T2: Encode, > Encode for (T0, T1, T2, ) {
    fn encode(&self, out: &mut impl io::Write) -> Result {
        out.write_u8(104)?;
        out.write_u8(3)?;
        self.0.encode(out)?;
        self.1.encode(out)?;
        self.2.encode(out)?;
        Ok(())
    }
}

impl<T0: Encode, T1: Encode, T2: Encode, T3: Encode, > Encode for (T0, T1, T2, T3, ) {
    fn encode(&self, out: &mut impl io::Write) -> Result {
        out.write_u8(104)?;
        out.write_u8(4)?;
        self.0.encode(out)?;
        self.1.encode(out)?;
        self.2.encode(out)?;
        self.3.encode(out)?;
        Ok(())
    }
}

impl<T0: Encode, T1: Encode, T2: Encode, T3: Encode, T4: Encode, > Encode for (T0, T1, T2, T3, T4, ) {
    fn encode(&self, out: &mut impl io::Write) -> Result {
        out.write_u8(104)?;
        out.write_u8(5)?;
        self.0.encode(out)?;
        self.1.encode(out)?;
        self.2.encode(out)?;
        self.3.encode(out)?;
        self.4.encode(out)?;
        Ok(())
    }
}

impl<T0: Encode, T1: Encode, T2: Encode, T3: Encode, T4: Encode, T5: Encode, > Encode for (T0, T1, T2, T3, T4, T5, ) {
    fn encode(&self, out: &mut impl io::Write) -> Result {
        out.write_u8(104)?;
        out.write_u8(6)?;
        self.0.encode(out)?;
        self.1.encode(out)?;
        self.2.encode(out)?;
        self.3.encode(out)?;
        self.4.encode(out)?;
        self.5.encode(out)?;
        Ok(())
    }
}

impl<T0: Encode, T1: Encode, T2: Encode, T3: Encode, T4: Encode, T5: Encode, T6: Encode, > Encode for (T0, T1, T2, T3, T4, T5, T6, ) {
    fn encode(&self, out: &mut impl io::Write) -> Result {
        out.write_u8(104)?;
        out.write_u8(7)?;
        self.0.encode(out)?;
        self.1.encode(out)?;
        self.2.encode(out)?;
        self.3.encode(out)?;
        self.4.encode(out)?;
        self.5.encode(out)?;
        self.6.encode(out)?;
        Ok(())
    }
}

impl<T0: Encode, T1: Encode, T2: Encode, T3: Encode, T4: Encode, T5: Encode, T6: Encode, T7: Encode, > Encode for (T0, T1, T2, T3, T4, T5, T6, T7, ) {
    fn encode(&self, out: &mut impl io::Write) -> Result {
        out.write_u8(104)?;
        out.write_u8(8)?;
        self.0.encode(out)?;
        self.1.encode(out)?;
        self.2.encode(out)?;
        self.3.encode(out)?;
        self.4.encode(out)?;
        self.5.encode(out)?;
        self.6.encode(out)?;
        self.7.encode(out)?;
        Ok(())
    }
}

impl<T0: Encode, T1: Encode, T2: Encode, T3: Encode, T4: Encode, T5: Encode, T6: Encode, T7: Encode, T8: Encode, > Encode for (T0, T1, T2, T3, T4, T5, T6, T7, T8, ) {
    fn encode(&self, out: &mut impl io::Write) -> Result {
        out.write_u8(104)?;
        out.write_u8(9)?;
        self.0.encode(out)?;
        self.1.encode(out)?;
        self.2.encode(out)?;
        self.3.encode(out)?;
        self.4.encode(out)?;
        self.5.encode(out)?;
        self.6.encode(out)?;
        self.7.encode(out)?;
        self.8.encode(out)?;
        Ok(())
    }
}

impl<T0: Encode, T1: Encode, T2: Encode, T3: Encode, T4: Encode, T5: Encode, T6: Encode, T7: Encode, T8: Encode, T9: Encode, > Encode for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, ) {
    fn encode(&self, out: &mut impl io::Write) -> Result {
        out.write_u8(104)?;
        out.write_u8(10)?;
        self.0.encode(out)?;
        self.1.encode(out)?;
        self.2.encode(out)?;
        self.3.encode(out)?;
        self.4.encode(out)?;
        self.5.encode(out)?;
        self.6.encode(out)?;
        self.7.encode(out)?;
        self.8.encode(out)?;
        self.9.encode(out)?;
        Ok(())
    }
}

impl<T0: Encode, T1: Encode, T2: Encode, T3: Encode, T4: Encode, T5: Encode, T6: Encode, T7: Encode, T8: Encode, T9: Encode, T10: Encode, > Encode for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, ) {
    fn encode(&self, out: &mut impl io::Write) -> Result {
        out.write_u8(104)?;
        out.write_u8(11)?;
        self.0.encode(out)?;
        self.1.encode(out)?;
        self.2.encode(out)?;
        self.3.encode(out)?;
        self.4.encode(out)?;
        self.5.encode(out)?;
        self.6.encode(out)?;
        self.7.encode(out)?;
        self.8.encode(out)?;
        self.9.encode(out)?;
        self.10.encode(out)?;
        Ok(())
    }
}

impl<T0: Encode, T1: Encode, T2: Encode, T3: Encode, T4: Encode, T5: Encode, T6: Encode, T7: Encode, T8: Encode, T9: Encode, T10: Encode, T11: Encode, > Encode for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, ) {
    fn encode(&self, out: &mut impl io::Write) -> Result {
        out.write_u8(104)?;
        out.write_u8(12)?;
        self.0.encode(out)?;
        self.1.encode(out)?;
        self.2.encode(out)?;
        self.3.encode(out)?;
        self.4.encode(out)?;
        self.5.encode(out)?;
        self.6.encode(out)?;
        self.7.encode(out)?;
        self.8.encode(out)?;
        self.9.encode(out)?;
        self.10.encode(out)?;
        self.11.encode(out)?;
        Ok(())
    }
}

impl<T0: Encode, T1: Encode, T2: Encode, T3: Encode, T4: Encode, T5: Encode, T6: Encode, T7: Encode, T8: Encode, T9: Encode, T10: Encode, T11: Encode, T12: Encode, > Encode for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, ) {
    fn encode(&self, out: &mut impl io::Write) -> Result {
        out.write_u8(104)?;
        out.write_u8(13)?;
        self.0.encode(out)?;
        self.1.encode(out)?;
        self.2.encode(out)?;
        self.3.encode(out)?;
        self.4.encode(out)?;
        self.5.encode(out)?;
        self.6.encode(out)?;
        self.7.encode(out)?;
        self.8.encode(out)?;
        self.9.encode(out)?;
        self.10.encode(out)?;
        self.11.encode(out)?;
        self.12.encode(out)?;
        Ok(())
    }
}

impl<T0: Encode, T1: Encode, T2: Encode, T3: Encode, T4: Encode, T5: Encode, T6: Encode, T7: Encode, T8: Encode, T9: Encode, T10: Encode, T11: Encode, T12: Encode, T13: Encode, > Encode for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, ) {
    fn encode(&self, out: &mut impl io::Write) -> Result {
        out.write_u8(104)?;
        out.write_u8(14)?;
        self.0.encode(out)?;
        self.1.encode(out)?;
        self.2.encode(out)?;
        self.3.encode(out)?;
        self.4.encode(out)?;
        self.5.encode(out)?;
        self.6.encode(out)?;
        self.7.encode(out)?;
        self.8.encode(out)?;
        self.9.encode(out)?;
        self.10.encode(out)?;
        self.11.encode(out)?;
        self.12.encode(out)?;
        self.13.encode(out)?;
        Ok(())
    }
}

impl<T0: Encode, T1: Encode, T2: Encode, T3: Encode, T4: Encode, T5: Encode, T6: Encode, T7: Encode, T8: Encode, T9: Encode, T10: Encode, T11: Encode, T12: Encode, T13: Encode, T14: Encode, > Encode for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, ) {
    fn encode(&self, out: &mut impl io::Write) -> Result {
        out.write_u8(104)?;
        out.write_u8(15)?;
        self.0.encode(out)?;
        self.1.encode(out)?;
        self.2.encode(out)?;
        self.3.encode(out)?;
        self.4.encode(out)?;
        self.5.encode(out)?;
        self.6.encode(out)?;
        self.7.encode(out)?;
        self.8.encode(out)?;
        self.9.encode(out)?;
        self.10.encode(out)?;
        self.11.encode(out)?;
        self.12.encode(out)?;
        self.13.encode(out)?;
        self.14.encode(out)?;
        Ok(())
    }
}

impl<T0: Encode, T1: Encode, T2: Encode, T3: Encode, T4: Encode, T5: Encode, T6: Encode, T7: Encode, T8: Encode, T9: Encode, T10: Encode, T11: Encode, T12: Encode, T13: Encode, T14: Encode, T15: Encode, > Encode for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, ) {
    fn encode(&self, out: &mut impl io::Write) -> Result {
        out.write_u8(104)?;
        out.write_u8(16)?;
        self.0.encode(out)?;
        self.1.encode(out)?;
        self.2.encode(out)?;
        self.3.encode(out)?;
        self.4.encode(out)?;
        self.5.encode(out)?;
        self.6.encode(out)?;
        self.7.encode(out)?;
        self.8.encode(out)?;
        self.9.encode(out)?;
        self.10.encode(out)?;
        self.11.encode(out)?;
        self.12.encode(out)?;
        self.13.encode(out)?;
        self.14.encode(out)?;
        self.15.encode(out)?;
        Ok(())
    }
}

impl<T0: Encode, T1: Encode, T2: Encode, T3: Encode, T4: Encode, T5: Encode, T6: Encode, T7: Encode, T8: Encode, T9: Encode, T10: Encode, T11: Encode, T12: Encode, T13: Encode, T14: Encode, T15: Encode, T16: Encode, > Encode for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, ) {
    fn encode(&self, out: &mut impl io::Write) -> Result {
        out.write_u8(104)?;
        out.write_u8(17)?;
        self.0.encode(out)?;
        self.1.encode(out)?;
        self.2.encode(out)?;
        self.3.encode(out)?;
        self.4.encode(out)?;
        self.5.encode(out)?;
        self.6.encode(out)?;
        self.7.encode(out)?;
        self.8.encode(out)?;
        self.9.encode(out)?;
        self.10.encode(out)?;
        self.11.encode(out)?;
        self.12.encode(out)?;
        self.13.encode(out)?;
        self.14.encode(out)?;
        self.15.encode(out)?;
        self.16.encode(out)?;
        Ok(())
    }
}

impl<T0: Encode, T1: Encode, T2: Encode, T3: Encode, T4: Encode, T5: Encode, T6: Encode, T7: Encode, T8: Encode, T9: Encode, T10: Encode, T11: Encode, T12: Encode, T13: Encode, T14: Encode, T15: Encode, T16: Encode, T17: Encode, > Encode for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, ) {
    fn encode(&self, out: &mut impl io::Write) -> Result {
        out.write_u8(104)?;
        out.write_u8(18)?;
        self.0.encode(out)?;
        self.1.encode(out)?;
        self.2.encode(out)?;
        self.3.encode(out)?;
        self.4.encode(out)?;
        self.5.encode(out)?;
        self.6.encode(out)?;
        self.7.encode(out)?;
        self.8.encode(out)?;
        self.9.encode(out)?;
        self.10.encode(out)?;
        self.11.encode(out)?;
        self.12.encode(out)?;
        self.13.encode(out)?;
        self.14.encode(out)?;
        self.15.encode(out)?;
        self.16.encode(out)?;
        self.17.encode(out)?;
        Ok(())
    }
}

impl<T0: Encode, T1: Encode, T2: Encode, T3: Encode, T4: Encode, T5: Encode, T6: Encode, T7: Encode, T8: Encode, T9: Encode, T10: Encode, T11: Encode, T12: Encode, T13: Encode, T14: Encode, T15: Encode, T16: Encode, T17: Encode, T18: Encode, > Encode for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, ) {
    fn encode(&self, out: &mut impl io::Write) -> Result {
        out.write_u8(104)?;
        out.write_u8(19)?;
        self.0.encode(out)?;
        self.1.encode(out)?;
        self.2.encode(out)?;
        self.3.encode(out)?;
        self.4.encode(out)?;
        self.5.encode(out)?;
        self.6.encode(out)?;
        self.7.encode(out)?;
        self.8.encode(out)?;
        self.9.encode(out)?;
        self.10.encode(out)?;
        self.11.encode(out)?;
        self.12.encode(out)?;
        self.13.encode(out)?;
        self.14.encode(out)?;
        self.15.encode(out)?;
        self.16.encode(out)?;
        self.17.encode(out)?;
        self.18.encode(out)?;
        Ok(())
    }
}

impl<T0: Encode, T1: Encode, T2: Encode, T3: Encode, T4: Encode, T5: Encode, T6: Encode, T7: Encode, T8: Encode, T9: Encode, T10: Encode, T11: Encode, T12: Encode, T13: Encode, T14: Encode, T15: Encode, T16: Encode, T17: Encode, T18: Encode, T19: Encode, > Encode for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, ) {
    fn encode(&self, out: &mut impl io::Write) -> Result {
        out.write_u8(104)?;
        out.write_u8(20)?;
        self.0.encode(out)?;
        self.1.encode(out)?;
        self.2.encode(out)?;
        self.3.encode(out)?;
        self.4.encode(out)?;
        self.5.encode(out)?;
        self.6.encode(out)?;
        self.7.encode(out)?;
        self.8.encode(out)?;
        self.9.encode(out)?;
        self.10.encode(out)?;
        self.11.encode(out)?;
        self.12.encode(out)?;
        self.13.encode(out)?;
        self.14.encode(out)?;
        self.15.encode(out)?;
        self.16.encode(out)?;
        self.17.encode(out)?;
        self.18.encode(out)?;
        self.19.encode(out)?;
        Ok(())
    }
}

impl<T0: Encode, T1: Encode, T2: Encode, T3: Encode, T4: Encode, T5: Encode, T6: Encode, T7: Encode, T8: Encode, T9: Encode, T10: Encode, T11: Encode, T12: Encode, T13: Encode, T14: Encode, T15: Encode, T16: Encode, T17: Encode, T18: Encode, T19: Encode, T20: Encode, > Encode for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, ) {
    fn encode(&self, out: &mut impl io::Write) -> Result {
        out.write_u8(104)?;
        out.write_u8(21)?;
        self.0.encode(out)?;
        self.1.encode(out)?;
        self.2.encode(out)?;
        self.3.encode(out)?;
        self.4.encode(out)?;
        self.5.encode(out)?;
        self.6.encode(out)?;
        self.7.encode(out)?;
        self.8.encode(out)?;
        self.9.encode(out)?;
        self.10.encode(out)?;
        self.11.encode(out)?;
        self.12.encode(out)?;
        self.13.encode(out)?;
        self.14.encode(out)?;
        self.15.encode(out)?;
        self.16.encode(out)?;
        self.17.encode(out)?;
        self.18.encode(out)?;
        self.19.encode(out)?;
        self.20.encode(out)?;
        Ok(())
    }
}

impl<T0: Encode, T1: Encode, T2: Encode, T3: Encode, T4: Encode, T5: Encode, T6: Encode, T7: Encode, T8: Encode, T9: Encode, T10: Encode, T11: Encode, T12: Encode, T13: Encode, T14: Encode, T15: Encode, T16: Encode, T17: Encode, T18: Encode, T19: Encode, T20: Encode, T21: Encode, > Encode for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, ) {
    fn encode(&self, out: &mut impl io::Write) -> Result {
        out.write_u8(104)?;
        out.write_u8(22)?;
        self.0.encode(out)?;
        self.1.encode(out)?;
        self.2.encode(out)?;
        self.3.encode(out)?;
        self.4.encode(out)?;
        self.5.encode(out)?;
        self.6.encode(out)?;
        self.7.encode(out)?;
        self.8.encode(out)?;
        self.9.encode(out)?;
        self.10.encode(out)?;
        self.11.encode(out)?;
        self.12.encode(out)?;
        self.13.encode(out)?;
        self.14.encode(out)?;
        self.15.encode(out)?;
        self.16.encode(out)?;
        self.17.encode(out)?;
        self.18.encode(out)?;
        self.19.encode(out)?;
        self.20.encode(out)?;
        self.21.encode(out)?;
        Ok(())
    }
}

impl<T0: Encode, T1: Encode, T2: Encode, T3: Encode, T4: Encode, T5: Encode, T6: Encode, T7: Encode, T8: Encode, T9: Encode, T10: Encode, T11: Encode, T12: Encode, T13: Encode, T14: Encode, T15: Encode, T16: Encode, T17: Encode, T18: Encode, T19: Encode, T20: Encode, T21: Encode, T22: Encode, > Encode for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, ) {
    fn encode(&self, out: &mut impl io::Write) -> Result {
        out.write_u8(104)?;
        out.write_u8(23)?;
        self.0.encode(out)?;
        self.1.encode(out)?;
        self.2.encode(out)?;
        self.3.encode(out)?;
        self.4.encode(out)?;
        self.5.encode(out)?;
        self.6.encode(out)?;
        self.7.encode(out)?;
        self.8.encode(out)?;
        self.9.encode(out)?;
        self.10.encode(out)?;
        self.11.encode(out)?;
        self.12.encode(out)?;
        self.13.encode(out)?;
        self.14.encode(out)?;
        self.15.encode(out)?;
        self.16.encode(out)?;
        self.17.encode(out)?;
        self.18.encode(out)?;
        self.19.encode(out)?;
        self.20.encode(out)?;
        self.21.encode(out)?;
        self.22.encode(out)?;
        Ok(())
    }
}

impl<T0: Encode, T1: Encode, T2: Encode, T3: Encode, T4: Encode, T5: Encode, T6: Encode, T7: Encode, T8: Encode, T9: Encode, T10: Encode, T11: Encode, T12: Encode, T13: Encode, T14: Encode, T15: Encode, T16: Encode, T17: Encode, T18: Encode, T19: Encode, T20: Encode, T21: Encode, T22: Encode, T23: Encode, > Encode for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, ) {
    fn encode(&self, out: &mut impl io::Write) -> Result {
        out.write_u8(104)?;
        out.write_u8(24)?;
        self.0.encode(out)?;
        self.1.encode(out)?;
        self.2.encode(out)?;
        self.3.encode(out)?;
        self.4.encode(out)?;
        self.5.encode(out)?;
        self.6.encode(out)?;
        self.7.encode(out)?;
        self.8.encode(out)?;
        self.9.encode(out)?;
        self.10.encode(out)?;
        self.11.encode(out)?;
        self.12.encode(out)?;
        self.13.encode(out)?;
        self.14.encode(out)?;
        self.15.encode(out)?;
        self.16.encode(out)?;
        self.17.encode(out)?;
        self.18.encode(out)?;
        self.19.encode(out)?;
        self.20.encode(out)?;
        self.21.encode(out)?;
        self.22.encode(out)?;
        self.23.encode(out)?;
        Ok(())
    }
}

impl<T0: Encode, T1: Encode, T2: Encode, T3: Encode, T4: Encode, T5: Encode, T6: Encode, T7: Encode, T8: Encode, T9: Encode, T10: Encode, T11: Encode, T12: Encode, T13: Encode, T14: Encode, T15: Encode, T16: Encode, T17: Encode, T18: Encode, T19: Encode, T20: Encode, T21: Encode, T22: Encode, T23: Encode, T24: Encode, > Encode for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, ) {
    fn encode(&self, out: &mut impl io::Write) -> Result {
        out.write_u8(104)?;
        out.write_u8(25)?;
        self.0.encode(out)?;
        self.1.encode(out)?;
        self.2.encode(out)?;
        self.3.encode(out)?;
        self.4.encode(out)?;
        self.5.encode(out)?;
        self.6.encode(out)?;
        self.7.encode(out)?;
        self.8.encode(out)?;
        self.9.encode(out)?;
        self.10.encode(out)?;
        self.11.encode(out)?;
        self.12.encode(out)?;
        self.13.encode(out)?;
        self.14.encode(out)?;
        self.15.encode(out)?;
        self.16.encode(out)?;
        self.17.encode(out)?;
        self.18.encode(out)?;
        self.19.encode(out)?;
        self.20.encode(out)?;
        self.21.encode(out)?;
        self.22.encode(out)?;
        self.23.encode(out)?;
        self.24.encode(out)?;
        Ok(())
    }
}

impl<T0: Encode, T1: Encode, T2: Encode, T3: Encode, T4: Encode, T5: Encode, T6: Encode, T7: Encode, T8: Encode, T9: Encode, T10: Encode, T11: Encode, T12: Encode, T13: Encode, T14: Encode, T15: Encode, T16: Encode, T17: Encode, T18: Encode, T19: Encode, T20: Encode, T21: Encode, T22: Encode, T23: Encode, T24: Encode, T25: Encode, > Encode for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, ) {
    fn encode(&self, out: &mut impl io::Write) -> Result {
        out.write_u8(104)?;
        out.write_u8(26)?;
        self.0.encode(out)?;
        self.1.encode(out)?;
        self.2.encode(out)?;
        self.3.encode(out)?;
        self.4.encode(out)?;
        self.5.encode(out)?;
        self.6.encode(out)?;
        self.7.encode(out)?;
        self.8.encode(out)?;
        self.9.encode(out)?;
        self.10.encode(out)?;
        self.11.encode(out)?;
        self.12.encode(out)?;
        self.13.encode(out)?;
        self.14.encode(out)?;
        self.15.encode(out)?;
        self.16.encode(out)?;
        self.17.encode(out)?;
        self.18.encode(out)?;
        self.19.encode(out)?;
        self.20.encode(out)?;
        self.21.encode(out)?;
        self.22.encode(out)?;
        self.23.encode(out)?;
        self.24.encode(out)?;
        self.25.encode(out)?;
        Ok(())
    }
}

impl<T0: Encode, T1: Encode, T2: Encode, T3: Encode, T4: Encode, T5: Encode, T6: Encode, T7: Encode, T8: Encode, T9: Encode, T10: Encode, T11: Encode, T12: Encode, T13: Encode, T14: Encode, T15: Encode, T16: Encode, T17: Encode, T18: Encode, T19: Encode, T20: Encode, T21: Encode, T22: Encode, T23: Encode, T24: Encode, T25: Encode, T26: Encode, > Encode for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, ) {
    fn encode(&self, out: &mut impl io::Write) -> Result {
        out.write_u8(104)?;
        out.write_u8(27)?;
        self.0.encode(out)?;
        self.1.encode(out)?;
        self.2.encode(out)?;
        self.3.encode(out)?;
        self.4.encode(out)?;
        self.5.encode(out)?;
        self.6.encode(out)?;
        self.7.encode(out)?;
        self.8.encode(out)?;
        self.9.encode(out)?;
        self.10.encode(out)?;
        self.11.encode(out)?;
        self.12.encode(out)?;
        self.13.encode(out)?;
        self.14.encode(out)?;
        self.15.encode(out)?;
        self.16.encode(out)?;
        self.17.encode(out)?;
        self.18.encode(out)?;
        self.19.encode(out)?;
        self.20.encode(out)?;
        self.21.encode(out)?;
        self.22.encode(out)?;
        self.23.encode(out)?;
        self.24.encode(out)?;
        self.25.encode(out)?;
        self.26.encode(out)?;
        Ok(())
    }
}

impl<T0: Encode, T1: Encode, T2: Encode, T3: Encode, T4: Encode, T5: Encode, T6: Encode, T7: Encode, T8: Encode, T9: Encode, T10: Encode, T11: Encode, T12: Encode, T13: Encode, T14: Encode, T15: Encode, T16: Encode, T17: Encode, T18: Encode, T19: Encode, T20: Encode, T21: Encode, T22: Encode, T23: Encode, T24: Encode, T25: Encode, T26: Encode, T27: Encode, > Encode for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, ) {
    fn encode(&self, out: &mut impl io::Write) -> Result {
        out.write_u8(104)?;
        out.write_u8(28)?;
        self.0.encode(out)?;
        self.1.encode(out)?;
        self.2.encode(out)?;
        self.3.encode(out)?;
        self.4.encode(out)?;
        self.5.encode(out)?;
        self.6.encode(out)?;
        self.7.encode(out)?;
        self.8.encode(out)?;
        self.9.encode(out)?;
        self.10.encode(out)?;
        self.11.encode(out)?;
        self.12.encode(out)?;
        self.13.encode(out)?;
        self.14.encode(out)?;
        self.15.encode(out)?;
        self.16.encode(out)?;
        self.17.encode(out)?;
        self.18.encode(out)?;
        self.19.encode(out)?;
        self.20.encode(out)?;
        self.21.encode(out)?;
        self.22.encode(out)?;
        self.23.encode(out)?;
        self.24.encode(out)?;
        self.25.encode(out)?;
        self.26.encode(out)?;
        self.27.encode(out)?;
        Ok(())
    }
}

impl<T0: Encode, T1: Encode, T2: Encode, T3: Encode, T4: Encode, T5: Encode, T6: Encode, T7: Encode, T8: Encode, T9: Encode, T10: Encode, T11: Encode, T12: Encode, T13: Encode, T14: Encode, T15: Encode, T16: Encode, T17: Encode, T18: Encode, T19: Encode, T20: Encode, T21: Encode, T22: Encode, T23: Encode, T24: Encode, T25: Encode, T26: Encode, T27: Encode, T28: Encode, > Encode for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, ) {
    fn encode(&self, out: &mut impl io::Write) -> Result {
        out.write_u8(104)?;
        out.write_u8(29)?;
        self.0.encode(out)?;
        self.1.encode(out)?;
        self.2.encode(out)?;
        self.3.encode(out)?;
        self.4.encode(out)?;
        self.5.encode(out)?;
        self.6.encode(out)?;
        self.7.encode(out)?;
        self.8.encode(out)?;
        self.9.encode(out)?;
        self.10.encode(out)?;
        self.11.encode(out)?;
        self.12.encode(out)?;
        self.13.encode(out)?;
        self.14.encode(out)?;
        self.15.encode(out)?;
        self.16.encode(out)?;
        self.17.encode(out)?;
        self.18.encode(out)?;
        self.19.encode(out)?;
        self.20.encode(out)?;
        self.21.encode(out)?;
        self.22.encode(out)?;
        self.23.encode(out)?;
        self.24.encode(out)?;
        self.25.encode(out)?;
        self.26.encode(out)?;
        self.27.encode(out)?;
        self.28.encode(out)?;
        Ok(())
    }
}

impl<T0: Encode, T1: Encode, T2: Encode, T3: Encode, T4: Encode, T5: Encode, T6: Encode, T7: Encode, T8: Encode, T9: Encode, T10: Encode, T11: Encode, T12: Encode, T13: Encode, T14: Encode, T15: Encode, T16: Encode, T17: Encode, T18: Encode, T19: Encode, T20: Encode, T21: Encode, T22: Encode, T23: Encode, T24: Encode, T25: Encode, T26: Encode, T27: Encode, T28: Encode, T29: Encode, > Encode for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, ) {
    fn encode(&self, out: &mut impl io::Write) -> Result {
        out.write_u8(104)?;
        out.write_u8(30)?;
        self.0.encode(out)?;
        self.1.encode(out)?;
        self.2.encode(out)?;
        self.3.encode(out)?;
        self.4.encode(out)?;
        self.5.encode(out)?;
        self.6.encode(out)?;
        self.7.encode(out)?;
        self.8.encode(out)?;
        self.9.encode(out)?;
        self.10.encode(out)?;
        self.11.encode(out)?;
        self.12.encode(out)?;
        self.13.encode(out)?;
        self.14.encode(out)?;
        self.15.encode(out)?;
        self.16.encode(out)?;
        self.17.encode(out)?;
        self.18.encode(out)?;
        self.19.encode(out)?;
        self.20.encode(out)?;
        self.21.encode(out)?;
        self.22.encode(out)?;
        self.23.encode(out)?;
        self.24.encode(out)?;
        self.25.encode(out)?;
        self.26.encode(out)?;
        self.27.encode(out)?;
        self.28.encode(out)?;
        self.29.encode(out)?;
        Ok(())
    }
}

impl<T0: Encode, T1: Encode, T2: Encode, T3: Encode, T4: Encode, T5: Encode, T6: Encode, T7: Encode, T8: Encode, T9: Encode, T10: Encode, T11: Encode, T12: Encode, T13: Encode, T14: Encode, T15: Encode, T16: Encode, T17: Encode, T18: Encode, T19: Encode, T20: Encode, T21: Encode, T22: Encode, T23: Encode, T24: Encode, T25: Encode, T26: Encode, T27: Encode, T28: Encode, T29: Encode, T30: Encode, > Encode for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, ) {
    fn encode(&self, out: &mut impl io::Write) -> Result {
        out.write_u8(104)?;
        out.write_u8(31)?;
        self.0.encode(out)?;
        self.1.encode(out)?;
        self.2.encode(out)?;
        self.3.encode(out)?;
        self.4.encode(out)?;
        self.5.encode(out)?;
        self.6.encode(out)?;
        self.7.encode(out)?;
        self.8.encode(out)?;
        self.9.encode(out)?;
        self.10.encode(out)?;
        self.11.encode(out)?;
        self.12.encode(out)?;
        self.13.encode(out)?;
        self.14.encode(out)?;
        self.15.encode(out)?;
        self.16.encode(out)?;
        self.17.encode(out)?;
        self.18.encode(out)?;
        self.19.encode(out)?;
        self.20.encode(out)?;
        self.21.encode(out)?;
        self.22.encode(out)?;
        self.23.encode(out)?;
        self.24.encode(out)?;
        self.25.encode(out)?;
        self.26.encode(out)?;
        self.27.encode(out)?;
        self.28.encode(out)?;
        self.29.encode(out)?;
        self.30.encode(out)?;
        Ok(())
    }
}

impl<T0: Encode, T1: Encode, T2: Encode, T3: Encode, T4: Encode, T5: Encode, T6: Encode, T7: Encode, T8: Encode, T9: Encode, T10: Encode, T11: Encode, T12: Encode, T13: Encode, T14: Encode, T15: Encode, T16: Encode, T17: Encode, T18: Encode, T19: Encode, T20: Encode, T21: Encode, T22: Encode, T23: Encode, T24: Encode, T25: Encode, T26: Encode, T27: Encode, T28: Encode, T29: Encode, T30: Encode, T31: Encode, > Encode for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, ) {
    fn encode(&self, out: &mut impl io::Write) -> Result {
        out.write_u8(104)?;
        out.write_u8(32)?;
        self.0.encode(out)?;
        self.1.encode(out)?;
        self.2.encode(out)?;
        self.3.encode(out)?;
        self.4.encode(out)?;
        self.5.encode(out)?;
        self.6.encode(out)?;
        self.7.encode(out)?;
        self.8.encode(out)?;
        self.9.encode(out)?;
        self.10.encode(out)?;
        self.11.encode(out)?;
        self.12.encode(out)?;
        self.13.encode(out)?;
        self.14.encode(out)?;
        self.15.encode(out)?;
        self.16.encode(out)?;
        self.17.encode(out)?;
        self.18.encode(out)?;
        self.19.encode(out)?;
        self.20.encode(out)?;
        self.21.encode(out)?;
        self.22.encode(out)?;
        self.23.encode(out)?;
        self.24.encode(out)?;
        self.25.encode(out)?;
        self.26.encode(out)?;
        self.27.encode(out)?;
        self.28.encode(out)?;
        self.29.encode(out)?;
        self.30.encode(out)?;
        self.31.encode(out)?;
        Ok(())
    }
}

