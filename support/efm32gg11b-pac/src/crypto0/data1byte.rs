#[doc = "Register `DATA1BYTE` reader"]
pub type R = crate::R<Data1byteSpec>;
#[doc = "Register `DATA1BYTE` writer"]
pub type W = crate::W<Data1byteSpec>;
#[doc = "Field `DATA1BYTE` reader - Data 1 Byte Access"]
pub type Data1byteR = crate::FieldReader;
#[doc = "Field `DATA1BYTE` writer - Data 1 Byte Access"]
pub type Data1byteW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Data 1 Byte Access"]
    #[inline(always)]
    pub fn data1byte(&self) -> Data1byteR {
        Data1byteR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data 1 Byte Access"]
    #[inline(always)]
    pub fn data1byte(&mut self) -> Data1byteW<'_, Data1byteSpec> {
        Data1byteW::new(self, 0)
    }
}
#[doc = "DATA1 Register Byte Access\n\nYou can [`read`](crate::Reg::read) this register and get [`data1byte::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data1byte::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\n<div class=\"warning\">One or more dependent resources other than the current register are immediately affected by a read operation.</div>"]
pub struct Data1byteSpec;
impl crate::RegisterSpec for Data1byteSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data1byte::R`](R) reader structure"]
impl crate::Readable for Data1byteSpec {}
#[doc = "`write(|w| ..)` method takes [`data1byte::W`](W) writer structure"]
impl crate::Writable for Data1byteSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DATA1BYTE to value 0"]
impl crate::Resettable for Data1byteSpec {}
