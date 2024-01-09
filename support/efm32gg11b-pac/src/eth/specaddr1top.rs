#[doc = "Register `SPECADDR1TOP` reader"]
pub type R = crate::R<SPECADDR1TOP_SPEC>;
#[doc = "Register `SPECADDR1TOP` writer"]
pub type W = crate::W<SPECADDR1TOP_SPEC>;
#[doc = "Field `ADDR` reader - Specific address 1 MSB"]
pub type ADDR_R = crate::FieldReader<u16>;
#[doc = "Field `ADDR` writer - Specific address 1 MSB"]
pub type ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `FILTERTYPE` reader - MAC SA or DA selection"]
pub type FILTERTYPE_R = crate::BitReader;
#[doc = "Field `FILTERTYPE` writer - MAC SA or DA selection"]
pub type FILTERTYPE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - Specific address 1 MSB"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - MAC SA or DA selection"]
    #[inline(always)]
    pub fn filtertype(&self) -> FILTERTYPE_R {
        FILTERTYPE_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Specific address 1 MSB"]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> ADDR_W<SPECADDR1TOP_SPEC> {
        ADDR_W::new(self, 0)
    }
    #[doc = "Bit 16 - MAC SA or DA selection"]
    #[inline(always)]
    #[must_use]
    pub fn filtertype(&mut self) -> FILTERTYPE_W<SPECADDR1TOP_SPEC> {
        FILTERTYPE_W::new(self, 16)
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
#[doc = "Specific Address 1 Top\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`specaddr1top::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`specaddr1top::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPECADDR1TOP_SPEC;
impl crate::RegisterSpec for SPECADDR1TOP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`specaddr1top::R`](R) reader structure"]
impl crate::Readable for SPECADDR1TOP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`specaddr1top::W`](W) writer structure"]
impl crate::Writable for SPECADDR1TOP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SPECADDR1TOP to value 0"]
impl crate::Resettable for SPECADDR1TOP_SPEC {
    const RESET_VALUE: u32 = 0;
}
