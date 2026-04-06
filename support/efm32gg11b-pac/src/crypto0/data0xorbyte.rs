#[doc = "Register `DATA0XORBYTE` reader"]
pub type R = crate::R<Data0xorbyteSpec>;
#[doc = "Register `DATA0XORBYTE` writer"]
pub type W = crate::W<Data0xorbyteSpec>;
#[doc = "Field `DATA0XORBYTE` reader - Data 0 XOR Byte Access"]
pub type Data0xorbyteR = crate::FieldReader;
#[doc = "Field `DATA0XORBYTE` writer - Data 0 XOR Byte Access"]
pub type Data0xorbyteW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Data 0 XOR Byte Access"]
    #[inline(always)]
    pub fn data0xorbyte(&self) -> Data0xorbyteR {
        Data0xorbyteR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data 0 XOR Byte Access"]
    #[inline(always)]
    pub fn data0xorbyte(&mut self) -> Data0xorbyteW<'_, Data0xorbyteSpec> {
        Data0xorbyteW::new(self, 0)
    }
}
#[doc = "DATA0 Register Byte XOR Access\n\nYou can [`read`](crate::Reg::read) this register and get [`data0xorbyte::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data0xorbyte::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\n<div class=\"warning\">One or more dependent resources other than the current register are immediately affected by a read operation.</div>"]
pub struct Data0xorbyteSpec;
impl crate::RegisterSpec for Data0xorbyteSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data0xorbyte::R`](R) reader structure"]
impl crate::Readable for Data0xorbyteSpec {}
#[doc = "`write(|w| ..)` method takes [`data0xorbyte::W`](W) writer structure"]
impl crate::Writable for Data0xorbyteSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DATA0XORBYTE to value 0"]
impl crate::Resettable for Data0xorbyteSpec {}
