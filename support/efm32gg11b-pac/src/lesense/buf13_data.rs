#[doc = "Register `BUF13_DATA` reader"]
pub type R = crate::R<BUF13_DATA_SPEC>;
#[doc = "Register `BUF13_DATA` writer"]
pub type W = crate::W<BUF13_DATA_SPEC>;
#[doc = "Field `DATA` reader - Scan Result Buffer"]
pub type DATA_R = crate::FieldReader<u16>;
#[doc = "Field `DATA` writer - Scan Result Buffer"]
pub type DATA_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `DATASRC` reader - Result Data Source"]
pub type DATASRC_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:15 - Scan Result Buffer"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:19 - Result Data Source"]
    #[inline(always)]
    pub fn datasrc(&self) -> DATASRC_R {
        DATASRC_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - Scan Result Buffer"]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DATA_W<BUF13_DATA_SPEC> {
        DATA_W::new(self, 0)
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
#[doc = "Scan Results\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`buf13_data::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`buf13_data::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BUF13_DATA_SPEC;
impl crate::RegisterSpec for BUF13_DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`buf13_data::R`](R) reader structure"]
impl crate::Readable for BUF13_DATA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`buf13_data::W`](W) writer structure"]
impl crate::Writable for BUF13_DATA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BUF13_DATA to value 0"]
impl crate::Resettable for BUF13_DATA_SPEC {
    const RESET_VALUE: u32 = 0;
}
