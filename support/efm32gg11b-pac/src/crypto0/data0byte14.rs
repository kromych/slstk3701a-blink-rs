#[doc = "Register `DATA0BYTE14` reader"]
pub type R = crate::R<Data0byte14Spec>;
#[doc = "Register `DATA0BYTE14` writer"]
pub type W = crate::W<Data0byte14Spec>;
#[doc = "Field `DATA0BYTE14` reader - Data 0 Byte 14 Access"]
pub type Data0byte14R = crate::FieldReader;
#[doc = "Field `DATA0BYTE14` writer - Data 0 Byte 14 Access"]
pub type Data0byte14W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Data 0 Byte 14 Access"]
    #[inline(always)]
    pub fn data0byte14(&self) -> Data0byte14R {
        Data0byte14R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data 0 Byte 14 Access"]
    #[inline(always)]
    pub fn data0byte14(&mut self) -> Data0byte14W<'_, Data0byte14Spec> {
        Data0byte14W::new(self, 0)
    }
}
#[doc = "DATA0 Register Byte 14 Access\n\nYou can [`read`](crate::Reg::read) this register and get [`data0byte14::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data0byte14::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Data0byte14Spec;
impl crate::RegisterSpec for Data0byte14Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data0byte14::R`](R) reader structure"]
impl crate::Readable for Data0byte14Spec {}
#[doc = "`write(|w| ..)` method takes [`data0byte14::W`](W) writer structure"]
impl crate::Writable for Data0byte14Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DATA0BYTE14 to value 0"]
impl crate::Resettable for Data0byte14Spec {}
