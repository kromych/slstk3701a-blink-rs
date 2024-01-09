#[doc = "Register `ETMCCR` reader"]
pub type R = crate::R<ETMCCR_SPEC>;
#[doc = "Field `ADRCMPPAIR` reader - Number of Address Comparator Pairs"]
pub type ADRCMPPAIR_R = crate::FieldReader;
#[doc = "Field `DATACMPNUM` reader - Number of Data Value Comparators"]
pub type DATACMPNUM_R = crate::FieldReader;
#[doc = "Field `MMDECCNT` reader - Number of Memeory Map Decoders"]
pub type MMDECCNT_R = crate::FieldReader;
#[doc = "Field `COUNTNUM` reader - Number of Counters"]
pub type COUNTNUM_R = crate::FieldReader;
#[doc = "Field `SEQPRES` reader - Sequencer Present"]
pub type SEQPRES_R = crate::BitReader;
#[doc = "Field `EXTINPNUM` reader - Number of External Inputs"]
pub type EXTINPNUM_R = crate::FieldReader<EXTINPNUM_A>;
#[doc = "Number of External Inputs\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTINPNUM_A {
    #[doc = "0: Zero inputs presents"]
    ZERO = 0,
    #[doc = "1: One inputs presents"]
    ONE = 1,
    #[doc = "2: Two inputs presents"]
    TWO = 2,
}
impl From<EXTINPNUM_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTINPNUM_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXTINPNUM_A {
    type Ux = u8;
}
impl EXTINPNUM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<EXTINPNUM_A> {
        match self.bits {
            0 => Some(EXTINPNUM_A::ZERO),
            1 => Some(EXTINPNUM_A::ONE),
            2 => Some(EXTINPNUM_A::TWO),
            _ => None,
        }
    }
    #[doc = "Zero inputs presents"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == EXTINPNUM_A::ZERO
    }
    #[doc = "One inputs presents"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == EXTINPNUM_A::ONE
    }
    #[doc = "Two inputs presents"]
    #[inline(always)]
    pub fn is_two(&self) -> bool {
        *self == EXTINPNUM_A::TWO
    }
}
#[doc = "Field `EXTOUTNUM` reader - Number of External Output"]
pub type EXTOUTNUM_R = crate::FieldReader;
#[doc = "Field `FIFOFULLPRES` reader - FIFIO FULL present"]
pub type FIFOFULLPRES_R = crate::BitReader;
#[doc = "Field `IDCOMPNUM` reader - Number of context ID Comparators"]
pub type IDCOMPNUM_R = crate::FieldReader;
#[doc = "Field `TRACESS` reader - Trace Start/Stop Block Present"]
pub type TRACESS_R = crate::BitReader;
#[doc = "Field `MMACCESS` reader - Coprocessor and Memeory Access"]
pub type MMACCESS_R = crate::BitReader;
#[doc = "Field `ETMID` reader - ETM ID Register Present"]
pub type ETMID_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:3 - Number of Address Comparator Pairs"]
    #[inline(always)]
    pub fn adrcmppair(&self) -> ADRCMPPAIR_R {
        ADRCMPPAIR_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Number of Data Value Comparators"]
    #[inline(always)]
    pub fn datacmpnum(&self) -> DATACMPNUM_R {
        DATACMPNUM_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:12 - Number of Memeory Map Decoders"]
    #[inline(always)]
    pub fn mmdeccnt(&self) -> MMDECCNT_R {
        MMDECCNT_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 13:15 - Number of Counters"]
    #[inline(always)]
    pub fn countnum(&self) -> COUNTNUM_R {
        COUNTNUM_R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bit 16 - Sequencer Present"]
    #[inline(always)]
    pub fn seqpres(&self) -> SEQPRES_R {
        SEQPRES_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:19 - Number of External Inputs"]
    #[inline(always)]
    pub fn extinpnum(&self) -> EXTINPNUM_R {
        EXTINPNUM_R::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bits 20:22 - Number of External Output"]
    #[inline(always)]
    pub fn extoutnum(&self) -> EXTOUTNUM_R {
        EXTOUTNUM_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 23 - FIFIO FULL present"]
    #[inline(always)]
    pub fn fifofullpres(&self) -> FIFOFULLPRES_R {
        FIFOFULLPRES_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:25 - Number of context ID Comparators"]
    #[inline(always)]
    pub fn idcompnum(&self) -> IDCOMPNUM_R {
        IDCOMPNUM_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 26 - Trace Start/Stop Block Present"]
    #[inline(always)]
    pub fn tracess(&self) -> TRACESS_R {
        TRACESS_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Coprocessor and Memeory Access"]
    #[inline(always)]
    pub fn mmaccess(&self) -> MMACCESS_R {
        MMACCESS_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 31 - ETM ID Register Present"]
    #[inline(always)]
    pub fn etmid(&self) -> ETMID_R {
        ETMID_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Configuration Code Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`etmccr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETMCCR_SPEC;
impl crate::RegisterSpec for ETMCCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`etmccr::R`](R) reader structure"]
impl crate::Readable for ETMCCR_SPEC {}
#[doc = "`reset()` method sets ETMCCR to value 0x8c80_2000"]
impl crate::Resettable for ETMCCR_SPEC {
    const RESET_VALUE: u32 = 0x8c80_2000;
}
