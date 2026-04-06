#[doc = "Register `DATA0BYTE13` reader"]
pub type R = crate::R<Data0byte13Spec>;
#[doc = "Register `DATA0BYTE13` writer"]
pub type W = crate::W<Data0byte13Spec>;
#[doc = "Field `DATA0BYTE13` reader - Data 0 Byte 13 Access"]
pub type Data0byte13R = crate::FieldReader;
#[doc = "Field `DATA0BYTE13` writer - Data 0 Byte 13 Access"]
pub type Data0byte13W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Data 0 Byte 13 Access"]
    #[inline(always)]
    pub fn data0byte13(&self) -> Data0byte13R {
        Data0byte13R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data 0 Byte 13 Access"]
    #[inline(always)]
    pub fn data0byte13(&mut self) -> Data0byte13W<'_, Data0byte13Spec> {
        Data0byte13W::new(self, 0)
    }
}
#[doc = "DATA0 Register Byte 13 Access\n\nYou can [`read`](crate::Reg::read) this register and get [`data0byte13::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data0byte13::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Data0byte13Spec;
impl crate::RegisterSpec for Data0byte13Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data0byte13::R`](R) reader structure"]
impl crate::Readable for Data0byte13Spec {}
#[doc = "`write(|w| ..)` method takes [`data0byte13::W`](W) writer structure"]
impl crate::Writable for Data0byte13Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DATA0BYTE13 to value 0"]
impl crate::Resettable for Data0byte13Spec {}
