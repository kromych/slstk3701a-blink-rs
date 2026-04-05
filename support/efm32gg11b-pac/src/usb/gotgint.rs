#[doc = "Register `GOTGINT` reader"]
pub type R = crate::R<GotgintSpec>;
#[doc = "Register `GOTGINT` writer"]
pub type W = crate::W<GotgintSpec>;
#[doc = "Field `SESENDDET` reader - Session End Detected"]
pub type SesenddetR = crate::BitReader;
#[doc = "Field `SESENDDET` writer - Session End Detected"]
pub type SesenddetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SESREQSUCSTSCHNG` reader - Session Request Success Status Change"]
pub type SesreqsucstschngR = crate::BitReader;
#[doc = "Field `SESREQSUCSTSCHNG` writer - Session Request Success Status Change"]
pub type SesreqsucstschngW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSTNEGSUCSTSCHNG` reader - Host Negotiation Success Status Change"]
pub type HstnegsucstschngR = crate::BitReader;
#[doc = "Field `HSTNEGSUCSTSCHNG` writer - Host Negotiation Success Status Change"]
pub type HstnegsucstschngW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSTNEGDET` reader - Host Negotiation Detected"]
pub type HstnegdetR = crate::BitReader;
#[doc = "Field `HSTNEGDET` writer - Host Negotiation Detected"]
pub type HstnegdetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADEVTOUTCHG` reader - A-Device Timeout Change"]
pub type AdevtoutchgR = crate::BitReader;
#[doc = "Field `ADEVTOUTCHG` writer - A-Device Timeout Change"]
pub type AdevtoutchgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBNCEDONE` reader - Debounce Done"]
pub type DbncedoneR = crate::BitReader;
#[doc = "Field `DBNCEDONE` writer - Debounce Done"]
pub type DbncedoneW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2 - Session End Detected"]
    #[inline(always)]
    pub fn sesenddet(&self) -> SesenddetR {
        SesenddetR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - Session Request Success Status Change"]
    #[inline(always)]
    pub fn sesreqsucstschng(&self) -> SesreqsucstschngR {
        SesreqsucstschngR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Host Negotiation Success Status Change"]
    #[inline(always)]
    pub fn hstnegsucstschng(&self) -> HstnegsucstschngR {
        HstnegsucstschngR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 17 - Host Negotiation Detected"]
    #[inline(always)]
    pub fn hstnegdet(&self) -> HstnegdetR {
        HstnegdetR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - A-Device Timeout Change"]
    #[inline(always)]
    pub fn adevtoutchg(&self) -> AdevtoutchgR {
        AdevtoutchgR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Debounce Done"]
    #[inline(always)]
    pub fn dbncedone(&self) -> DbncedoneR {
        DbncedoneR::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Session End Detected"]
    #[inline(always)]
    pub fn sesenddet(&mut self) -> SesenddetW<'_, GotgintSpec> {
        SesenddetW::new(self, 2)
    }
    #[doc = "Bit 8 - Session Request Success Status Change"]
    #[inline(always)]
    pub fn sesreqsucstschng(&mut self) -> SesreqsucstschngW<'_, GotgintSpec> {
        SesreqsucstschngW::new(self, 8)
    }
    #[doc = "Bit 9 - Host Negotiation Success Status Change"]
    #[inline(always)]
    pub fn hstnegsucstschng(&mut self) -> HstnegsucstschngW<'_, GotgintSpec> {
        HstnegsucstschngW::new(self, 9)
    }
    #[doc = "Bit 17 - Host Negotiation Detected"]
    #[inline(always)]
    pub fn hstnegdet(&mut self) -> HstnegdetW<'_, GotgintSpec> {
        HstnegdetW::new(self, 17)
    }
    #[doc = "Bit 18 - A-Device Timeout Change"]
    #[inline(always)]
    pub fn adevtoutchg(&mut self) -> AdevtoutchgW<'_, GotgintSpec> {
        AdevtoutchgW::new(self, 18)
    }
    #[doc = "Bit 19 - Debounce Done"]
    #[inline(always)]
    pub fn dbncedone(&mut self) -> DbncedoneW<'_, GotgintSpec> {
        DbncedoneW::new(self, 19)
    }
}
#[doc = "OTG Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gotgint::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gotgint::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GotgintSpec;
impl crate::RegisterSpec for GotgintSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gotgint::R`](R) reader structure"]
impl crate::Readable for GotgintSpec {}
#[doc = "`write(|w| ..)` method takes [`gotgint::W`](W) writer structure"]
impl crate::Writable for GotgintSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GOTGINT to value 0"]
impl crate::Resettable for GotgintSpec {}
