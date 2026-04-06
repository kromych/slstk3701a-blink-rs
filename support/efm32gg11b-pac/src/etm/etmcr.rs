#[doc = "Register `ETMCR` reader"]
pub type R = crate::R<EtmcrSpec>;
#[doc = "Register `ETMCR` writer"]
pub type W = crate::W<EtmcrSpec>;
#[doc = "Field `POWERDWN` reader - ETM Control in low power mode"]
pub type PowerdwnR = crate::BitReader;
#[doc = "Field `POWERDWN` writer - ETM Control in low power mode"]
pub type PowerdwnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PORTSIZE` reader - ETM Port Size"]
pub type PortsizeR = crate::FieldReader;
#[doc = "Field `PORTSIZE` writer - ETM Port Size"]
pub type PortsizeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `STALL` reader - Stall Processor"]
pub type StallR = crate::BitReader;
#[doc = "Field `STALL` writer - Stall Processor"]
pub type StallW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BRANCHOUTPUT` reader - Branch Output"]
pub type BranchoutputR = crate::BitReader;
#[doc = "Field `BRANCHOUTPUT` writer - Branch Output"]
pub type BranchoutputW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBGREQCTRL` reader - Debug Request Control"]
pub type DbgreqctrlR = crate::BitReader;
#[doc = "Field `DBGREQCTRL` writer - Debug Request Control"]
pub type DbgreqctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETMPROG` reader - ETM Programming"]
pub type EtmprogR = crate::BitReader;
#[doc = "Field `ETMPROG` writer - ETM Programming"]
pub type EtmprogW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETMPORTSEL` reader - ETM Port Selection"]
pub type EtmportselR = crate::BitReader;
#[doc = "Field `ETMPORTSEL` writer - ETM Port Selection"]
pub type EtmportselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PORTMODE2` reader - Port Mode\\[2\\]"]
pub type Portmode2R = crate::BitReader;
#[doc = "Field `PORTMODE2` writer - Port Mode\\[2\\]"]
pub type Portmode2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PORTMODE` reader - Port Mode Control"]
pub type PortmodeR = crate::FieldReader;
#[doc = "Field `PORTMODE` writer - Port Mode Control"]
pub type PortmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EPORTSIZE` reader - Port Size\\[3\\]"]
pub type EportsizeR = crate::FieldReader;
#[doc = "Field `EPORTSIZE` writer - Port Size\\[3\\]"]
pub type EportsizeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TSTAMPEN` reader - Time Stamp Enable"]
pub type TstampenR = crate::BitReader;
#[doc = "Field `TSTAMPEN` writer - Time Stamp Enable"]
pub type TstampenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - ETM Control in low power mode"]
    #[inline(always)]
    pub fn powerdwn(&self) -> PowerdwnR {
        PowerdwnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:6 - ETM Port Size"]
    #[inline(always)]
    pub fn portsize(&self) -> PortsizeR {
        PortsizeR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Stall Processor"]
    #[inline(always)]
    pub fn stall(&self) -> StallR {
        StallR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Branch Output"]
    #[inline(always)]
    pub fn branchoutput(&self) -> BranchoutputR {
        BranchoutputR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Debug Request Control"]
    #[inline(always)]
    pub fn dbgreqctrl(&self) -> DbgreqctrlR {
        DbgreqctrlR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - ETM Programming"]
    #[inline(always)]
    pub fn etmprog(&self) -> EtmprogR {
        EtmprogR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - ETM Port Selection"]
    #[inline(always)]
    pub fn etmportsel(&self) -> EtmportselR {
        EtmportselR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - Port Mode\\[2\\]"]
    #[inline(always)]
    pub fn portmode2(&self) -> Portmode2R {
        Portmode2R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Port Mode Control"]
    #[inline(always)]
    pub fn portmode(&self) -> PortmodeR {
        PortmodeR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 21:22 - Port Size\\[3\\]"]
    #[inline(always)]
    pub fn eportsize(&self) -> EportsizeR {
        EportsizeR::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bit 28 - Time Stamp Enable"]
    #[inline(always)]
    pub fn tstampen(&self) -> TstampenR {
        TstampenR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ETM Control in low power mode"]
    #[inline(always)]
    pub fn powerdwn(&mut self) -> PowerdwnW<'_, EtmcrSpec> {
        PowerdwnW::new(self, 0)
    }
    #[doc = "Bits 4:6 - ETM Port Size"]
    #[inline(always)]
    pub fn portsize(&mut self) -> PortsizeW<'_, EtmcrSpec> {
        PortsizeW::new(self, 4)
    }
    #[doc = "Bit 7 - Stall Processor"]
    #[inline(always)]
    pub fn stall(&mut self) -> StallW<'_, EtmcrSpec> {
        StallW::new(self, 7)
    }
    #[doc = "Bit 8 - Branch Output"]
    #[inline(always)]
    pub fn branchoutput(&mut self) -> BranchoutputW<'_, EtmcrSpec> {
        BranchoutputW::new(self, 8)
    }
    #[doc = "Bit 9 - Debug Request Control"]
    #[inline(always)]
    pub fn dbgreqctrl(&mut self) -> DbgreqctrlW<'_, EtmcrSpec> {
        DbgreqctrlW::new(self, 9)
    }
    #[doc = "Bit 10 - ETM Programming"]
    #[inline(always)]
    pub fn etmprog(&mut self) -> EtmprogW<'_, EtmcrSpec> {
        EtmprogW::new(self, 10)
    }
    #[doc = "Bit 11 - ETM Port Selection"]
    #[inline(always)]
    pub fn etmportsel(&mut self) -> EtmportselW<'_, EtmcrSpec> {
        EtmportselW::new(self, 11)
    }
    #[doc = "Bit 13 - Port Mode\\[2\\]"]
    #[inline(always)]
    pub fn portmode2(&mut self) -> Portmode2W<'_, EtmcrSpec> {
        Portmode2W::new(self, 13)
    }
    #[doc = "Bits 16:17 - Port Mode Control"]
    #[inline(always)]
    pub fn portmode(&mut self) -> PortmodeW<'_, EtmcrSpec> {
        PortmodeW::new(self, 16)
    }
    #[doc = "Bits 21:22 - Port Size\\[3\\]"]
    #[inline(always)]
    pub fn eportsize(&mut self) -> EportsizeW<'_, EtmcrSpec> {
        EportsizeW::new(self, 21)
    }
    #[doc = "Bit 28 - Time Stamp Enable"]
    #[inline(always)]
    pub fn tstampen(&mut self) -> TstampenW<'_, EtmcrSpec> {
        TstampenW::new(self, 28)
    }
}
#[doc = "Main Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`etmcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`etmcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EtmcrSpec;
impl crate::RegisterSpec for EtmcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`etmcr::R`](R) reader structure"]
impl crate::Readable for EtmcrSpec {}
#[doc = "`write(|w| ..)` method takes [`etmcr::W`](W) writer structure"]
impl crate::Writable for EtmcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ETMCR to value 0x0411"]
impl crate::Resettable for EtmcrSpec {
    const RESET_VALUE: u32 = 0x0411;
}
