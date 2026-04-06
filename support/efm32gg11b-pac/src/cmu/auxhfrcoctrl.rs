#[doc = "Register `AUXHFRCOCTRL` reader"]
pub type R = crate::R<AuxhfrcoctrlSpec>;
#[doc = "Register `AUXHFRCOCTRL` writer"]
pub type W = crate::W<AuxhfrcoctrlSpec>;
#[doc = "Field `TUNING` reader - AUXHFRCO Tuning Value"]
pub type TuningR = crate::FieldReader;
#[doc = "Field `TUNING` writer - AUXHFRCO Tuning Value"]
pub type TuningW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `FINETUNING` reader - AUXHFRCO Fine Tuning Value"]
pub type FinetuningR = crate::FieldReader;
#[doc = "Field `FINETUNING` writer - AUXHFRCO Fine Tuning Value"]
pub type FinetuningW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `FREQRANGE` reader - AUXHFRCO Frequency Range"]
pub type FreqrangeR = crate::FieldReader;
#[doc = "Field `FREQRANGE` writer - AUXHFRCO Frequency Range"]
pub type FreqrangeW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CMPBIAS` reader - AUXHFRCO Comparator Bias Current"]
pub type CmpbiasR = crate::FieldReader;
#[doc = "Field `CMPBIAS` writer - AUXHFRCO Comparator Bias Current"]
pub type CmpbiasW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `LDOHP` reader - AUXHFRCO LDO High Power Mode"]
pub type LdohpR = crate::BitReader;
#[doc = "Field `LDOHP` writer - AUXHFRCO LDO High Power Mode"]
pub type LdohpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Locally Divide AUXHFRCO Clock Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Clkdiv {
    #[doc = "0: Divide by 1."]
    Div1 = 0,
    #[doc = "1: Divide by 2."]
    Div2 = 1,
    #[doc = "2: Divide by 4."]
    Div4 = 2,
}
impl From<Clkdiv> for u8 {
    #[inline(always)]
    fn from(variant: Clkdiv) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Clkdiv {
    type Ux = u8;
}
impl crate::IsEnum for Clkdiv {}
#[doc = "Field `CLKDIV` reader - Locally Divide AUXHFRCO Clock Output"]
pub type ClkdivR = crate::FieldReader<Clkdiv>;
impl ClkdivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Clkdiv> {
        match self.bits {
            0 => Some(Clkdiv::Div1),
            1 => Some(Clkdiv::Div2),
            2 => Some(Clkdiv::Div4),
            _ => None,
        }
    }
    #[doc = "Divide by 1."]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == Clkdiv::Div1
    }
    #[doc = "Divide by 2."]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == Clkdiv::Div2
    }
    #[doc = "Divide by 4."]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == Clkdiv::Div4
    }
}
#[doc = "Field `CLKDIV` writer - Locally Divide AUXHFRCO Clock Output"]
pub type ClkdivW<'a, REG> = crate::FieldWriter<'a, REG, 2, Clkdiv>;
impl<'a, REG> ClkdivW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Divide by 1."]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(Clkdiv::Div1)
    }
    #[doc = "Divide by 2."]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(Clkdiv::Div2)
    }
    #[doc = "Divide by 4."]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(Clkdiv::Div4)
    }
}
#[doc = "Field `FINETUNINGEN` reader - Enable Reference for Fine Tuning"]
pub type FinetuningenR = crate::BitReader;
#[doc = "Field `FINETUNINGEN` writer - Enable Reference for Fine Tuning"]
pub type FinetuningenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VREFTC` reader - AUXHFRCO Temperature Coefficient Trim on Comparator Reference"]
pub type VreftcR = crate::FieldReader;
#[doc = "Field `VREFTC` writer - AUXHFRCO Temperature Coefficient Trim on Comparator Reference"]
pub type VreftcW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:6 - AUXHFRCO Tuning Value"]
    #[inline(always)]
    pub fn tuning(&self) -> TuningR {
        TuningR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:13 - AUXHFRCO Fine Tuning Value"]
    #[inline(always)]
    pub fn finetuning(&self) -> FinetuningR {
        FinetuningR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:20 - AUXHFRCO Frequency Range"]
    #[inline(always)]
    pub fn freqrange(&self) -> FreqrangeR {
        FreqrangeR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 21:23 - AUXHFRCO Comparator Bias Current"]
    #[inline(always)]
    pub fn cmpbias(&self) -> CmpbiasR {
        CmpbiasR::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bit 24 - AUXHFRCO LDO High Power Mode"]
    #[inline(always)]
    pub fn ldohp(&self) -> LdohpR {
        LdohpR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:26 - Locally Divide AUXHFRCO Clock Output"]
    #[inline(always)]
    pub fn clkdiv(&self) -> ClkdivR {
        ClkdivR::new(((self.bits >> 25) & 3) as u8)
    }
    #[doc = "Bit 27 - Enable Reference for Fine Tuning"]
    #[inline(always)]
    pub fn finetuningen(&self) -> FinetuningenR {
        FinetuningenR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:31 - AUXHFRCO Temperature Coefficient Trim on Comparator Reference"]
    #[inline(always)]
    pub fn vreftc(&self) -> VreftcR {
        VreftcR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - AUXHFRCO Tuning Value"]
    #[inline(always)]
    pub fn tuning(&mut self) -> TuningW<'_, AuxhfrcoctrlSpec> {
        TuningW::new(self, 0)
    }
    #[doc = "Bits 8:13 - AUXHFRCO Fine Tuning Value"]
    #[inline(always)]
    pub fn finetuning(&mut self) -> FinetuningW<'_, AuxhfrcoctrlSpec> {
        FinetuningW::new(self, 8)
    }
    #[doc = "Bits 16:20 - AUXHFRCO Frequency Range"]
    #[inline(always)]
    pub fn freqrange(&mut self) -> FreqrangeW<'_, AuxhfrcoctrlSpec> {
        FreqrangeW::new(self, 16)
    }
    #[doc = "Bits 21:23 - AUXHFRCO Comparator Bias Current"]
    #[inline(always)]
    pub fn cmpbias(&mut self) -> CmpbiasW<'_, AuxhfrcoctrlSpec> {
        CmpbiasW::new(self, 21)
    }
    #[doc = "Bit 24 - AUXHFRCO LDO High Power Mode"]
    #[inline(always)]
    pub fn ldohp(&mut self) -> LdohpW<'_, AuxhfrcoctrlSpec> {
        LdohpW::new(self, 24)
    }
    #[doc = "Bits 25:26 - Locally Divide AUXHFRCO Clock Output"]
    #[inline(always)]
    pub fn clkdiv(&mut self) -> ClkdivW<'_, AuxhfrcoctrlSpec> {
        ClkdivW::new(self, 25)
    }
    #[doc = "Bit 27 - Enable Reference for Fine Tuning"]
    #[inline(always)]
    pub fn finetuningen(&mut self) -> FinetuningenW<'_, AuxhfrcoctrlSpec> {
        FinetuningenW::new(self, 27)
    }
    #[doc = "Bits 28:31 - AUXHFRCO Temperature Coefficient Trim on Comparator Reference"]
    #[inline(always)]
    pub fn vreftc(&mut self) -> VreftcW<'_, AuxhfrcoctrlSpec> {
        VreftcW::new(self, 28)
    }
}
#[doc = "AUXHFRCO Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`auxhfrcoctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`auxhfrcoctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AuxhfrcoctrlSpec;
impl crate::RegisterSpec for AuxhfrcoctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`auxhfrcoctrl::R`](R) reader structure"]
impl crate::Readable for AuxhfrcoctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`auxhfrcoctrl::W`](W) writer structure"]
impl crate::Writable for AuxhfrcoctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AUXHFRCOCTRL to value 0xb148_1f7f"]
impl crate::Resettable for AuxhfrcoctrlSpec {
    const RESET_VALUE: u32 = 0xb148_1f7f;
}
