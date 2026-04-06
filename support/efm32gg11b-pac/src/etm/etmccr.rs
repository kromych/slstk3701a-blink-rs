#[doc = "Register `ETMCCR` reader"]
pub type R = crate::R<EtmccrSpec>;
#[doc = "Field `ADRCMPPAIR` reader - Number of Address Comparator Pairs"]
pub type AdrcmppairR = crate::FieldReader;
#[doc = "Field `DATACMPNUM` reader - Number of Data Value Comparators"]
pub type DatacmpnumR = crate::FieldReader;
#[doc = "Field `MMDECCNT` reader - Number of Memeory Map Decoders"]
pub type MmdeccntR = crate::FieldReader;
#[doc = "Field `COUNTNUM` reader - Number of Counters"]
pub type CountnumR = crate::FieldReader;
#[doc = "Field `SEQPRES` reader - Sequencer Present"]
pub type SeqpresR = crate::BitReader;
#[doc = "Number of External Inputs\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Extinpnum {
    #[doc = "0: Zero inputs presents"]
    Zero = 0,
    #[doc = "1: One inputs presents"]
    One = 1,
    #[doc = "2: Two inputs presents"]
    Two = 2,
}
impl From<Extinpnum> for u8 {
    #[inline(always)]
    fn from(variant: Extinpnum) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Extinpnum {
    type Ux = u8;
}
impl crate::IsEnum for Extinpnum {}
#[doc = "Field `EXTINPNUM` reader - Number of External Inputs"]
pub type ExtinpnumR = crate::FieldReader<Extinpnum>;
impl ExtinpnumR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Extinpnum> {
        match self.bits {
            0 => Some(Extinpnum::Zero),
            1 => Some(Extinpnum::One),
            2 => Some(Extinpnum::Two),
            _ => None,
        }
    }
    #[doc = "Zero inputs presents"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == Extinpnum::Zero
    }
    #[doc = "One inputs presents"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == Extinpnum::One
    }
    #[doc = "Two inputs presents"]
    #[inline(always)]
    pub fn is_two(&self) -> bool {
        *self == Extinpnum::Two
    }
}
#[doc = "Field `EXTOUTNUM` reader - Number of External Output"]
pub type ExtoutnumR = crate::FieldReader;
#[doc = "Field `FIFOFULLPRES` reader - FIFIO FULL present"]
pub type FifofullpresR = crate::BitReader;
#[doc = "Field `IDCOMPNUM` reader - Number of context ID Comparators"]
pub type IdcompnumR = crate::FieldReader;
#[doc = "Field `TRACESS` reader - Trace Start/Stop Block Present"]
pub type TracessR = crate::BitReader;
#[doc = "Field `MMACCESS` reader - Coprocessor and Memeory Access"]
pub type MmaccessR = crate::BitReader;
#[doc = "Field `ETMID` reader - ETM ID Register Present"]
pub type EtmidR = crate::BitReader;
impl R {
    #[doc = "Bits 0:3 - Number of Address Comparator Pairs"]
    #[inline(always)]
    pub fn adrcmppair(&self) -> AdrcmppairR {
        AdrcmppairR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Number of Data Value Comparators"]
    #[inline(always)]
    pub fn datacmpnum(&self) -> DatacmpnumR {
        DatacmpnumR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:12 - Number of Memeory Map Decoders"]
    #[inline(always)]
    pub fn mmdeccnt(&self) -> MmdeccntR {
        MmdeccntR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 13:15 - Number of Counters"]
    #[inline(always)]
    pub fn countnum(&self) -> CountnumR {
        CountnumR::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bit 16 - Sequencer Present"]
    #[inline(always)]
    pub fn seqpres(&self) -> SeqpresR {
        SeqpresR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:19 - Number of External Inputs"]
    #[inline(always)]
    pub fn extinpnum(&self) -> ExtinpnumR {
        ExtinpnumR::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bits 20:22 - Number of External Output"]
    #[inline(always)]
    pub fn extoutnum(&self) -> ExtoutnumR {
        ExtoutnumR::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 23 - FIFIO FULL present"]
    #[inline(always)]
    pub fn fifofullpres(&self) -> FifofullpresR {
        FifofullpresR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:25 - Number of context ID Comparators"]
    #[inline(always)]
    pub fn idcompnum(&self) -> IdcompnumR {
        IdcompnumR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 26 - Trace Start/Stop Block Present"]
    #[inline(always)]
    pub fn tracess(&self) -> TracessR {
        TracessR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Coprocessor and Memeory Access"]
    #[inline(always)]
    pub fn mmaccess(&self) -> MmaccessR {
        MmaccessR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 31 - ETM ID Register Present"]
    #[inline(always)]
    pub fn etmid(&self) -> EtmidR {
        EtmidR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Configuration Code Register\n\nYou can [`read`](crate::Reg::read) this register and get [`etmccr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EtmccrSpec;
impl crate::RegisterSpec for EtmccrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`etmccr::R`](R) reader structure"]
impl crate::Readable for EtmccrSpec {}
#[doc = "`reset()` method sets ETMCCR to value 0x8c80_2000"]
impl crate::Resettable for EtmccrSpec {
    const RESET_VALUE: u32 = 0x8c80_2000;
}
