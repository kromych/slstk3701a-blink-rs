#[doc = "Register `IFS` writer"]
pub type W = crate::W<IFS_SPEC>;
#[doc = "Field `FPIOC` writer - Set FPIOC Interrupt Flag"]
pub type FPIOC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPDZC` writer - Set FPDZC Interrupt Flag"]
pub type FPDZC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPUFC` writer - Set FPUFC Interrupt Flag"]
pub type FPUFC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPOFC` writer - Set FPOFC Interrupt Flag"]
pub type FPOFC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPIDC` writer - Set FPIDC Interrupt Flag"]
pub type FPIDC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPIXC` writer - Set FPIXC Interrupt Flag"]
pub type FPIXC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Set FPIOC Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn fpioc(&mut self) -> FPIOC_W<IFS_SPEC> {
        FPIOC_W::new(self, 0)
    }
    #[doc = "Bit 1 - Set FPDZC Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn fpdzc(&mut self) -> FPDZC_W<IFS_SPEC> {
        FPDZC_W::new(self, 1)
    }
    #[doc = "Bit 2 - Set FPUFC Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn fpufc(&mut self) -> FPUFC_W<IFS_SPEC> {
        FPUFC_W::new(self, 2)
    }
    #[doc = "Bit 3 - Set FPOFC Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn fpofc(&mut self) -> FPOFC_W<IFS_SPEC> {
        FPOFC_W::new(self, 3)
    }
    #[doc = "Bit 4 - Set FPIDC Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn fpidc(&mut self) -> FPIDC_W<IFS_SPEC> {
        FPIDC_W::new(self, 4)
    }
    #[doc = "Bit 5 - Set FPIXC Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn fpixc(&mut self) -> FPIXC_W<IFS_SPEC> {
        FPIXC_W::new(self, 5)
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
#[doc = "Interrupt Flag Set Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ifs::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IFS_SPEC;
impl crate::RegisterSpec for IFS_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ifs::W`](W) writer structure"]
impl crate::Writable for IFS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IFS to value 0"]
impl crate::Resettable for IFS_SPEC {
    const RESET_VALUE: u32 = 0;
}
