#[doc = "Register `IEN` reader"]
pub type R = crate::R<IEN_SPEC>;
#[doc = "Register `IEN` writer"]
pub type W = crate::W<IEN_SPEC>;
#[doc = "Field `CMP` reader - CMP Interrupt Enable"]
pub type CMP_R = crate::BitReader;
#[doc = "Field `CMP` writer - CMP Interrupt Enable"]
pub type CMP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CONV` reader - CONV Interrupt Enable"]
pub type CONV_R = crate::BitReader;
#[doc = "Field `CONV` writer - CONV Interrupt Enable"]
pub type CONV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOS` reader - EOS Interrupt Enable"]
pub type EOS_R = crate::BitReader;
#[doc = "Field `EOS` writer - EOS Interrupt Enable"]
pub type EOS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAOF` reader - DMAOF Interrupt Enable"]
pub type DMAOF_R = crate::BitReader;
#[doc = "Field `DMAOF` writer - DMAOF Interrupt Enable"]
pub type DMAOF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APORTCONFLICT` reader - APORTCONFLICT Interrupt Enable"]
pub type APORTCONFLICT_R = crate::BitReader;
#[doc = "Field `APORTCONFLICT` writer - APORTCONFLICT Interrupt Enable"]
pub type APORTCONFLICT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - CMP Interrupt Enable"]
    #[inline(always)]
    pub fn cmp(&self) -> CMP_R {
        CMP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CONV Interrupt Enable"]
    #[inline(always)]
    pub fn conv(&self) -> CONV_R {
        CONV_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - EOS Interrupt Enable"]
    #[inline(always)]
    pub fn eos(&self) -> EOS_R {
        EOS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMAOF Interrupt Enable"]
    #[inline(always)]
    pub fn dmaof(&self) -> DMAOF_R {
        DMAOF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - APORTCONFLICT Interrupt Enable"]
    #[inline(always)]
    pub fn aportconflict(&self) -> APORTCONFLICT_R {
        APORTCONFLICT_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CMP Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmp(&mut self) -> CMP_W<IEN_SPEC> {
        CMP_W::new(self, 0)
    }
    #[doc = "Bit 1 - CONV Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn conv(&mut self) -> CONV_W<IEN_SPEC> {
        CONV_W::new(self, 1)
    }
    #[doc = "Bit 2 - EOS Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn eos(&mut self) -> EOS_W<IEN_SPEC> {
        EOS_W::new(self, 2)
    }
    #[doc = "Bit 3 - DMAOF Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmaof(&mut self) -> DMAOF_W<IEN_SPEC> {
        DMAOF_W::new(self, 3)
    }
    #[doc = "Bit 4 - APORTCONFLICT Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn aportconflict(&mut self) -> APORTCONFLICT_W<IEN_SPEC> {
        APORTCONFLICT_W::new(self, 4)
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
#[doc = "Interrupt Enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ien::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ien::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
