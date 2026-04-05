#[doc = "Register `DDATA0BYTE32` reader"]
pub type R = crate::R<Ddata0byte32Spec>;
#[doc = "Register `DDATA0BYTE32` writer"]
pub type W = crate::W<Ddata0byte32Spec>;
#[doc = "Field `DDATA0BYTE32` reader - Ddata 0 Byte 32 Access"]
pub type Ddata0byte32R = crate::FieldReader;
#[doc = "Field `DDATA0BYTE32` writer - Ddata 0 Byte 32 Access"]
pub type Ddata0byte32W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Ddata 0 Byte 32 Access"]
    #[inline(always)]
    pub fn ddata0byte32(&self) -> Ddata0byte32R {
        Ddata0byte32R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Ddata 0 Byte 32 Access"]
    #[inline(always)]
    pub fn ddata0byte32(&mut self) -> Ddata0byte32W<'_, Ddata0byte32Spec> {
        Ddata0byte32W::new(self, 0)
    }
}
#[doc = "DDATA0 Register Byte 32 Access\n\nYou can [`read`](crate::Reg::read) this register and get [`ddata0byte32::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddata0byte32::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ddata0byte32Spec;
impl crate::RegisterSpec for Ddata0byte32Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddata0byte32::R`](R) reader structure"]
impl crate::Readable for Ddata0byte32Spec {}
#[doc = "`write(|w| ..)` method takes [`ddata0byte32::W`](W) writer structure"]
impl crate::Writable for Ddata0byte32Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DDATA0BYTE32 to value 0"]
impl crate::Resettable for Ddata0byte32Spec {}
