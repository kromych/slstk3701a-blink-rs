#[doc = "Register `ETMSR` reader"]
pub type R = crate::R<ETMSR_SPEC>;
#[doc = "Register `ETMSR` writer"]
pub type W = crate::W<ETMSR_SPEC>;
#[doc = "Field `ETHOF` reader - ETM Overflow"]
pub type ETHOF_R = crate::BitReader;
#[doc = "Field `ETMPROGBIT` reader - ETM Programming Bit Status"]
pub type ETMPROGBIT_R = crate::BitReader;
#[doc = "Field `TRACESTAT` reader - Trace Start/Stop Status"]
pub type TRACESTAT_R = crate::BitReader;
#[doc = "Field `TRACESTAT` writer - Trace Start/Stop Status"]
pub type TRACESTAT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRIGBIT` reader - Trigger Bit"]
pub type TRIGBIT_R = crate::BitReader;
#[doc = "Field `TRIGBIT` writer - Trigger Bit"]
pub type TRIGBIT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - ETM Overflow"]
    #[inline(always)]
    pub fn ethof(&self) -> ETHOF_R {
        ETHOF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ETM Programming Bit Status"]
    #[inline(always)]
    pub fn etmprogbit(&self) -> ETMPROGBIT_R {
        ETMPROGBIT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Trace Start/Stop Status"]
    #[inline(always)]
    pub fn tracestat(&self) -> TRACESTAT_R {
        TRACESTAT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Trigger Bit"]
    #[inline(always)]
    pub fn trigbit(&self) -> TRIGBIT_R {
        TRIGBIT_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Trace Start/Stop Status"]
    #[inline(always)]
    #[must_use]
    pub fn tracestat(&mut self) -> TRACESTAT_W<ETMSR_SPEC> {
        TRACESTAT_W::new(self, 2)
    }
    #[doc = "Bit 3 - Trigger Bit"]
    #[inline(always)]
    #[must_use]
    pub fn trigbit(&mut self) -> TRIGBIT_W<ETMSR_SPEC> {
        TRIGBIT_W::new(self, 3)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "ETM Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`etmsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`etmsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETMSR_SPEC;
impl crate::RegisterSpec for ETMSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`etmsr::R`](R) reader structure"]
impl crate::Readable for ETMSR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`etmsr::W`](W) writer structure"]
impl crate::Writable for ETMSR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ETMSR to value 0x02"]
impl crate::Resettable for ETMSR_SPEC {
    const RESET_VALUE: u32 = 0x02;
}
