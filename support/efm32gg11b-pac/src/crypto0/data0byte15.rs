#[doc = "Register `DATA0BYTE15` reader"]
pub type R = crate::R<Data0byte15Spec>;
#[doc = "Register `DATA0BYTE15` writer"]
pub type W = crate::W<Data0byte15Spec>;
#[doc = "Field `DATA0BYTE15` reader - Data 0 Byte 15 Access"]
pub type Data0byte15R = crate::FieldReader;
#[doc = "Field `DATA0BYTE15` writer - Data 0 Byte 15 Access"]
pub type Data0byte15W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Data 0 Byte 15 Access"]
    #[inline(always)]
    pub fn data0byte15(&self) -> Data0byte15R {
        Data0byte15R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data 0 Byte 15 Access"]
    #[inline(always)]
    pub fn data0byte15(&mut self) -> Data0byte15W<'_, Data0byte15Spec> {
        Data0byte15W::new(self, 0)
    }
}
#[doc = "DATA0 Register Byte 15 Access\n\nYou can [`read`](crate::Reg::read) this register and get [`data0byte15::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data0byte15::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Data0byte15Spec;
impl crate::RegisterSpec for Data0byte15Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data0byte15::R`](R) reader structure"]
impl crate::Readable for Data0byte15Spec {}
#[doc = "`write(|w| ..)` method takes [`data0byte15::W`](W) writer structure"]
impl crate::Writable for Data0byte15Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DATA0BYTE15 to value 0"]
impl crate::Resettable for Data0byte15Spec {}
