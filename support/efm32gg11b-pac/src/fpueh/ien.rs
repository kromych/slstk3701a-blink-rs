#[doc = "Register `IEN` reader"]
pub type R = crate::R<IEN_SPEC>;
#[doc = "Register `IEN` writer"]
pub type W = crate::W<IEN_SPEC>;
#[doc = "Field `FPIOC` reader - FPIOC Interrupt Enable"]
pub type FPIOC_R = crate::BitReader;
#[doc = "Field `FPIOC` writer - FPIOC Interrupt Enable"]
pub type FPIOC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPDZC` reader - FPDZC Interrupt Enable"]
pub type FPDZC_R = crate::BitReader;
#[doc = "Field `FPDZC` writer - FPDZC Interrupt Enable"]
pub type FPDZC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPUFC` reader - FPUFC Interrupt Enable"]
pub type FPUFC_R = crate::BitReader;
#[doc = "Field `FPUFC` writer - FPUFC Interrupt Enable"]
pub type FPUFC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPOFC` reader - FPOFC Interrupt Enable"]
pub type FPOFC_R = crate::BitReader;
#[doc = "Field `FPOFC` writer - FPOFC Interrupt Enable"]
pub type FPOFC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPIDC` reader - FPIDC Interrupt Enable"]
pub type FPIDC_R = crate::BitReader;
#[doc = "Field `FPIDC` writer - FPIDC Interrupt Enable"]
pub type FPIDC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPIXC` reader - FPIXC Interrupt Enable"]
pub type FPIXC_R = crate::BitReader;
#[doc = "Field `FPIXC` writer - FPIXC Interrupt Enable"]
pub type FPIXC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - FPIOC Interrupt Enable"]
    #[inline(always)]
    pub fn fpioc(&self) -> FPIOC_R {
        FPIOC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - FPDZC Interrupt Enable"]
    #[inline(always)]
    pub fn fpdzc(&self) -> FPDZC_R {
        FPDZC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - FPUFC Interrupt Enable"]
    #[inline(always)]
    pub fn fpufc(&self) -> FPUFC_R {
        FPUFC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - FPOFC Interrupt Enable"]
    #[inline(always)]
    pub fn fpofc(&self) -> FPOFC_R {
        FPOFC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - FPIDC Interrupt Enable"]
    #[inline(always)]
    pub fn fpidc(&self) -> FPIDC_R {
        FPIDC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - FPIXC Interrupt Enable"]
    #[inline(always)]
    pub fn fpixc(&self) -> FPIXC_R {
        FPIXC_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FPIOC Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fpioc(&mut self) -> FPIOC_W<IEN_SPEC> {
        FPIOC_W::new(self, 0)
    }
    #[doc = "Bit 1 - FPDZC Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fpdzc(&mut self) -> FPDZC_W<IEN_SPEC> {
        FPDZC_W::new(self, 1)
    }
    #[doc = "Bit 2 - FPUFC Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fpufc(&mut self) -> FPUFC_W<IEN_SPEC> {
        FPUFC_W::new(self, 2)
    }
    #[doc = "Bit 3 - FPOFC Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fpofc(&mut self) -> FPOFC_W<IEN_SPEC> {
        FPOFC_W::new(self, 3)
    }
    #[doc = "Bit 4 - FPIDC Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fpidc(&mut self) -> FPIDC_W<IEN_SPEC> {
        FPIDC_W::new(self, 4)
    }
    #[doc = "Bit 5 - FPIXC Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fpixc(&mut self) -> FPIXC_W<IEN_SPEC> {
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
#[doc = "Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ien::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ien::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IEN_SPEC;
impl crate::RegisterSpec for IEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ien::R`](R) reader structure"]
impl crate::Readable for IEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ien::W`](W) writer structure"]
impl crate::Writable for IEN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IEN to value 0"]
impl crate::Resettable for IEN_SPEC {
    const RESET_VALUE: u32 = 0;
}
