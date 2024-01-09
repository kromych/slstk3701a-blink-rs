#[doc = "Register `SRAMPARTITIONCFG` reader"]
pub type R = crate::R<SRAMPARTITIONCFG_SPEC>;
#[doc = "Register `SRAMPARTITIONCFG` writer"]
pub type W = crate::W<SRAMPARTITIONCFG_SPEC>;
#[doc = "Field `ADDR` reader - Indirect Read Partition Size"]
pub type ADDR_R = crate::FieldReader;
#[doc = "Field `ADDR` writer - Indirect Read Partition Size"]
pub type ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Indirect Read Partition Size"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Indirect Read Partition Size"]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> ADDR_W<SRAMPARTITIONCFG_SPEC> {
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
#[doc = "SRAM Partition Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srampartitioncfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srampartitioncfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRAMPARTITIONCFG_SPEC;
impl crate::RegisterSpec for SRAMPARTITIONCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`srampartitioncfg::R`](R) reader structure"]
impl crate::Readable for SRAMPARTITIONCFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`srampartitioncfg::W`](W) writer structure"]
impl crate::Writable for SRAMPARTITIONCFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SRAMPARTITIONCFG to value 0x80"]
impl crate::Resettable for SRAMPARTITIONCFG_SPEC {
    const RESET_VALUE: u32 = 0x80;
}
