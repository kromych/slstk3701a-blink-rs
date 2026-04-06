#[doc = "Register `DATA0BYTE` reader"]
pub type R = crate::R<Data0byteSpec>;
#[doc = "Register `DATA0BYTE` writer"]
pub type W = crate::W<Data0byteSpec>;
#[doc = "Field `DATA0BYTE` reader - Data 0 Byte Access"]
pub type Data0byteR = crate::FieldReader;
#[doc = "Field `DATA0BYTE` writer - Data 0 Byte Access"]
pub type Data0byteW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Data 0 Byte Access"]
    #[inline(always)]
    pub fn data0byte(&self) -> Data0byteR {
        Data0byteR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data 0 Byte Access"]
    #[inline(always)]
    pub fn data0byte(&mut self) -> Data0byteW<'_, Data0byteSpec> {
        Data0byteW::new(self, 0)
    }
}
#[doc = "DATA0 Register Byte Access\n\nYou can [`read`](crate::Reg::read) this register and get [`data0byte::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data0byte::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\n<div class=\"warning\">One or more dependent resources other than the current register are immediately affected by a read operation.</div>"]
pub struct Data0byteSpec;
impl crate::RegisterSpec for Data0byteSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data0byte::R`](R) reader structure"]
impl crate::Readable for Data0byteSpec {}
#[doc = "`write(|w| ..)` method takes [`data0byte::W`](W) writer structure"]
impl crate::Writable for Data0byteSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DATA0BYTE to value 0"]
impl crate::Resettable for Data0byteSpec {}
