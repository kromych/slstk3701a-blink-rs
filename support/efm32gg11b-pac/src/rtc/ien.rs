#[doc = "Register `IEN` reader"]
pub type R = crate::R<IEN_SPEC>;
#[doc = "Register `IEN` writer"]
pub type W = crate::W<IEN_SPEC>;
#[doc = "Field `OF` reader - OF Interrupt Enable"]
pub type OF_R = crate::BitReader;
#[doc = "Field `OF` writer - OF Interrupt Enable"]
pub type OF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMP` reader - COMP Interrupt Enable"]
pub type COMP_R = crate::FieldReader;
#[doc = "Field `COMP` writer - COMP Interrupt Enable"]
pub type COMP_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bit 0 - OF Interrupt Enable"]
    #[inline(always)]
    pub fn of(&self) -> OF_R {
        OF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:6 - COMP Interrupt Enable"]
    #[inline(always)]
    pub fn comp(&self) -> COMP_R {
        COMP_R::new(((self.bits >> 1) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - OF Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn of(&mut self) -> OF_W<IEN_SPEC> {
        OF_W::new(self, 0)
    }
    #[doc = "Bits 1:6 - COMP Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn comp(&mut self) -> COMP_W<IEN_SPEC> {
        COMP_W::new(self, 1)
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
