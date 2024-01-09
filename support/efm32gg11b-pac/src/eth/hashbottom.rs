#[doc = "Register `HASHBOTTOM` reader"]
pub type R = crate::R<HASHBOTTOM_SPEC>;
#[doc = "Register `HASHBOTTOM` writer"]
pub type W = crate::W<HASHBOTTOM_SPEC>;
#[doc = "Field `ADDR` reader - The first 32 bits of the hash address register."]
pub type ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `ADDR` writer - The first 32 bits of the hash address register."]
pub type ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - The first 32 bits of the hash address register."]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - The first 32 bits of the hash address register."]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> ADDR_W<HASHBOTTOM_SPEC> {
        ADDR_W::new(self, 0)
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
#[doc = "Hash Register Bottom \\[31:0\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hashbottom::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hashbottom::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HASHBOTTOM_SPEC;
impl crate::RegisterSpec for HASHBOTTOM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hashbottom::R`](R) reader structure"]
impl crate::Readable for HASHBOTTOM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hashbottom::W`](W) writer structure"]
impl crate::Writable for HASHBOTTOM_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HASHBOTTOM to value 0"]
impl crate::Resettable for HASHBOTTOM_SPEC {
    const RESET_VALUE: u32 = 0;
}
