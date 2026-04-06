#[doc = "Register `TEST` reader"]
pub type R = crate::R<TestSpec>;
#[doc = "Register `TEST` writer"]
pub type W = crate::W<TestSpec>;
#[doc = "Field `BASIC` reader - Basic Mode"]
pub type BasicR = crate::BitReader;
#[doc = "Field `BASIC` writer - Basic Mode"]
pub type BasicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SILENT` reader - Silent Mode"]
pub type SilentR = crate::BitReader;
#[doc = "Field `SILENT` writer - Silent Mode"]
pub type SilentW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LBACK` reader - Loopback Mode"]
pub type LbackR = crate::BitReader;
#[doc = "Field `LBACK` writer - Loopback Mode"]
pub type LbackW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Control of CAN_TX Pin\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tx {
    #[doc = "0: Reset value, CAN_TX is controlled by the CAN Core."]
    Core = 0,
    #[doc = "1: Sample Point can be monitored at CAN_TX pin."]
    Sampt = 1,
    #[doc = "2: CAN_TX pin drives a dominant bit (0) value."]
    Low = 2,
    #[doc = "3: CAN_TX pin drives a recessive bit (1) value."]
    High = 3,
}
impl From<Tx> for u8 {
    #[inline(always)]
    fn from(variant: Tx) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tx {
    type Ux = u8;
}
impl crate::IsEnum for Tx {}
#[doc = "Field `TX` reader - Control of CAN_TX Pin"]
pub type TxR = crate::FieldReader<Tx>;
impl TxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tx {
        match self.bits {
            0 => Tx::Core,
            1 => Tx::Sampt,
            2 => Tx::Low,
            3 => Tx::High,
            _ => unreachable!(),
        }
    }
    #[doc = "Reset value, CAN_TX is controlled by the CAN Core."]
    #[inline(always)]
    pub fn is_core(&self) -> bool {
        *self == Tx::Core
    }
    #[doc = "Sample Point can be monitored at CAN_TX pin."]
    #[inline(always)]
    pub fn is_sampt(&self) -> bool {
        *self == Tx::Sampt
    }
    #[doc = "CAN_TX pin drives a dominant bit (0) value."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Tx::Low
    }
    #[doc = "CAN_TX pin drives a recessive bit (1) value."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Tx::High
    }
}
#[doc = "Field `TX` writer - Control of CAN_TX Pin"]
pub type TxW<'a, REG> = crate::FieldWriter<'a, REG, 2, Tx, crate::Safe>;
impl<'a, REG> TxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Reset value, CAN_TX is controlled by the CAN Core."]
    #[inline(always)]
    pub fn core(self) -> &'a mut crate::W<REG> {
        self.variant(Tx::Core)
    }
    #[doc = "Sample Point can be monitored at CAN_TX pin."]
    #[inline(always)]
    pub fn sampt(self) -> &'a mut crate::W<REG> {
        self.variant(Tx::Sampt)
    }
    #[doc = "CAN_TX pin drives a dominant bit (0) value."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Tx::Low)
    }
    #[doc = "CAN_TX pin drives a recessive bit (1) value."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Tx::High)
    }
}
#[doc = "Field `RX` reader - Monitors the Actual Value of CAN_RX Pin"]
pub type RxR = crate::BitReader;
impl R {
    #[doc = "Bit 2 - Basic Mode"]
    #[inline(always)]
    pub fn basic(&self) -> BasicR {
        BasicR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Silent Mode"]
    #[inline(always)]
    pub fn silent(&self) -> SilentR {
        SilentR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Loopback Mode"]
    #[inline(always)]
    pub fn lback(&self) -> LbackR {
        LbackR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Control of CAN_TX Pin"]
    #[inline(always)]
    pub fn tx(&self) -> TxR {
        TxR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Monitors the Actual Value of CAN_RX Pin"]
    #[inline(always)]
    pub fn rx(&self) -> RxR {
        RxR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Basic Mode"]
    #[inline(always)]
    pub fn basic(&mut self) -> BasicW<'_, TestSpec> {
        BasicW::new(self, 2)
    }
    #[doc = "Bit 3 - Silent Mode"]
    #[inline(always)]
    pub fn silent(&mut self) -> SilentW<'_, TestSpec> {
        SilentW::new(self, 3)
    }
    #[doc = "Bit 4 - Loopback Mode"]
    #[inline(always)]
    pub fn lback(&mut self) -> LbackW<'_, TestSpec> {
        LbackW::new(self, 4)
    }
    #[doc = "Bits 5:6 - Control of CAN_TX Pin"]
    #[inline(always)]
    pub fn tx(&mut self) -> TxW<'_, TestSpec> {
        TxW::new(self, 5)
    }
}
#[doc = "Test Register\n\nYou can [`read`](crate::Reg::read) this register and get [`test::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`test::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TestSpec;
impl crate::RegisterSpec for TestSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`test::R`](R) reader structure"]
impl crate::Readable for TestSpec {}
#[doc = "`write(|w| ..)` method takes [`test::W`](W) writer structure"]
impl crate::Writable for TestSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TEST to value 0"]
impl crate::Resettable for TestSpec {}
