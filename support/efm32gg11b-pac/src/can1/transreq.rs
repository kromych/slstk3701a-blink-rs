#[doc = "Register `TRANSREQ` reader"]
pub type R = crate::R<TRANSREQ_SPEC>;
#[doc = "Field `TXRQSTOUT` reader - Transmission Request Bits (Of All Message Objects)"]
pub type TXRQSTOUT_R = crate::FieldReader<TXRQSTOUT_A>;
#[doc = "Transmission Request Bits (Of All Message Objects)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum TXRQSTOUT_A {
    #[doc = "0: This Message Object is not waiting for transmission."]
    FALSE = 0,
    #[doc = "1: The transmission of this Message Object is requested and is not yet done."]
    TRUE = 1,
}
impl From<TXRQSTOUT_A> for u32 {
    #[inline(always)]
    fn from(variant: TXRQSTOUT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TXRQSTOUT_A {
    type Ux = u32;
}
impl TXRQSTOUT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TXRQSTOUT_A> {
        match self.bits {
            0 => Some(TXRQSTOUT_A::FALSE),
            1 => Some(TXRQSTOUT_A::TRUE),
            _ => None,
        }
    }
    #[doc = "This Message Object is not waiting for transmission."]
    #[inline(always)]
    pub fn is_false(&self) -> bool {
        *self == TXRQSTOUT_A::FALSE
    }
    #[doc = "The transmission of this Message Object is requested and is not yet done."]
    #[inline(always)]
    pub fn is_true(&self) -> bool {
        *self == TXRQSTOUT_A::TRUE
    }
}
impl R {
    #[doc = "Bits 0:31 - Transmission Request Bits (Of All Message Objects)"]
    #[inline(always)]
    pub fn txrqstout(&self) -> TXRQSTOUT_R {
        TXRQSTOUT_R::new(self.bits)
    }
}
#[doc = "Transmission Request Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`transreq::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TRANSREQ_SPEC;
impl crate::RegisterSpec for TRANSREQ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`transreq::R`](R) reader structure"]
impl crate::Readable for TRANSREQ_SPEC {}
#[doc = "`reset()` method sets TRANSREQ to value 0"]
impl crate::Resettable for TRANSREQ_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
