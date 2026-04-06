#[doc = "Register `TRANSREQ` reader"]
pub type R = crate::R<TransreqSpec>;
#[doc = "Transmission Request Bits (Of All Message Objects)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Txrqstout {
    #[doc = "0: This Message Object is not waiting for transmission."]
    False = 0,
    #[doc = "1: The transmission of this Message Object is requested and is not yet done."]
    True = 1,
}
impl From<Txrqstout> for u32 {
    #[inline(always)]
    fn from(variant: Txrqstout) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Txrqstout {
    type Ux = u32;
}
impl crate::IsEnum for Txrqstout {}
#[doc = "Field `TXRQSTOUT` reader - Transmission Request Bits (Of All Message Objects)"]
pub type TxrqstoutR = crate::FieldReader<Txrqstout>;
impl TxrqstoutR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Txrqstout> {
        match self.bits {
            0 => Some(Txrqstout::False),
            1 => Some(Txrqstout::True),
            _ => None,
        }
    }
    #[doc = "This Message Object is not waiting for transmission."]
    #[inline(always)]
    pub fn is_false(&self) -> bool {
        *self == Txrqstout::False
    }
    #[doc = "The transmission of this Message Object is requested and is not yet done."]
    #[inline(always)]
    pub fn is_true(&self) -> bool {
        *self == Txrqstout::True
    }
}
impl R {
    #[doc = "Bits 0:31 - Transmission Request Bits (Of All Message Objects)"]
    #[inline(always)]
    pub fn txrqstout(&self) -> TxrqstoutR {
        TxrqstoutR::new(self.bits)
    }
}
#[doc = "Transmission Request Register\n\nYou can [`read`](crate::Reg::read) this register and get [`transreq::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TransreqSpec;
impl crate::RegisterSpec for TransreqSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`transreq::R`](R) reader structure"]
impl crate::Readable for TransreqSpec {}
#[doc = "`reset()` method sets TRANSREQ to value 0"]
impl crate::Resettable for TransreqSpec {}
