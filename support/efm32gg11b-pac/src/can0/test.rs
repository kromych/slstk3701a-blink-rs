#[doc = "Register `TEST` reader"]
pub type R = crate::R<TEST_SPEC>;
#[doc = "Register `TEST` writer"]
pub type W = crate::W<TEST_SPEC>;
#[doc = "Field `BASIC` reader - Basic Mode"]
pub type BASIC_R = crate::BitReader;
#[doc = "Field `BASIC` writer - Basic Mode"]
pub type BASIC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SILENT` reader - Silent Mode"]
pub type SILENT_R = crate::BitReader;
#[doc = "Field `SILENT` writer - Silent Mode"]
pub type SILENT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LBACK` reader - Loopback Mode"]
pub type LBACK_R = crate::BitReader;
#[doc = "Field `LBACK` writer - Loopback Mode"]
pub type LBACK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TX` reader - Control of CAN_TX Pin"]
pub type TX_R = crate::FieldReader<TX_A>;
#[doc = "Control of CAN_TX Pin\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TX_A {
    #[doc = "0: Reset value, CAN_TX is controlled by the CAN Core."]
    CORE = 0,
    #[doc = "1: Sample Point can be monitored at CAN_TX pin."]
    SAMPT = 1,
    #[doc = "2: CAN_TX pin drives a dominant bit (0) value."]
    LOW = 2,
    #[doc = "3: CAN_TX pin drives a recessive bit (1) value."]
    HIGH = 3,
}
impl From<TX_A> for u8 {
    #[inline(always)]
    fn from(variant: TX_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TX_A {
    type Ux = u8;
}
impl TX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TX_A {
        match self.bits {
            0 => TX_A::CORE,
            1 => TX_A::SAMPT,
            2 => TX_A::LOW,
            3 => TX_A::HIGH,
            _ => unreachable!(),
        }
    }
    #[doc = "Reset value, CAN_TX is controlled by the CAN Core."]
    #[inline(always)]
    pub fn is_core(&self) -> bool {
        *self == TX_A::CORE
    }
    #[doc = "Sample Point can be monitored at CAN_TX pin."]
    #[inline(always)]
    pub fn is_sampt(&self) -> bool {
        *self == TX_A::SAMPT
    }
    #[doc = "CAN_TX pin drives a dominant bit (0) value."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == TX_A::LOW
    }
    #[doc = "CAN_TX pin drives a recessive bit (1) value."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == TX_A::HIGH
    }
}
#[doc = "Field `TX` writer - Control of CAN_TX Pin"]
pub type TX_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, TX_A>;
impl<'a, REG, const O: u8> TX_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Reset value, CAN_TX is controlled by the CAN Core."]
    #[inline(always)]
    pub fn core(self) -> &'a mut crate::W<REG> {
        self.variant(TX_A::CORE)
    }
    #[doc = "Sample Point can be monitored at CAN_TX pin."]
    #[inline(always)]
    pub fn sampt(self) -> &'a mut crate::W<REG> {
        self.variant(TX_A::SAMPT)
    }
    #[doc = "CAN_TX pin drives a dominant bit (0) value."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(TX_A::LOW)
    }
    #[doc = "CAN_TX pin drives a recessive bit (1) value."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(TX_A::HIGH)
    }
}
#[doc = "Field `RX` reader - Monitors the Actual Value of CAN_RX Pin"]
pub type RX_R = crate::BitReader;
impl R {
    #[doc = "Bit 2 - Basic Mode"]
    #[inline(always)]
    pub fn basic(&self) -> BASIC_R {
        BASIC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Silent Mode"]
    #[inline(always)]
    pub fn silent(&self) -> SILENT_R {
        SILENT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Loopback Mode"]
    #[inline(always)]
    pub fn lback(&self) -> LBACK_R {
        LBACK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Control of CAN_TX Pin"]
    #[inline(always)]
    pub fn tx(&self) -> TX_R {
        TX_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Monitors the Actual Value of CAN_RX Pin"]
    #[inline(always)]
    pub fn rx(&self) -> RX_R {
        RX_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Basic Mode"]
    #[inline(always)]
    #[must_use]
    pub fn basic(&mut self) -> BASIC_W<TEST_SPEC, 2> {
        BASIC_W::new(self)
    }
    #[doc = "Bit 3 - Silent Mode"]
    #[inline(always)]
    #[must_use]
    pub fn silent(&mut self) -> SILENT_W<TEST_SPEC, 3> {
        SILENT_W::new(self)
    }
    #[doc = "Bit 4 - Loopback Mode"]
    #[inline(always)]
    #[must_use]
    pub fn lback(&mut self) -> LBACK_W<TEST_SPEC, 4> {
        LBACK_W::new(self)
    }
    #[doc = "Bits 5:6 - Control of CAN_TX Pin"]
    #[inline(always)]
    #[must_use]
    pub fn tx(&mut self) -> TX_W<TEST_SPEC, 5> {
        TX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Test Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`test::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`test::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TEST_SPEC;
impl crate::RegisterSpec for TEST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`test::R`](R) reader structure"]
impl crate::Readable for TEST_SPEC {}
#[doc = "`write(|w| ..)` method takes [`test::W`](W) writer structure"]
impl crate::Writable for TEST_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TEST to value 0"]
impl crate::Resettable for TEST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
