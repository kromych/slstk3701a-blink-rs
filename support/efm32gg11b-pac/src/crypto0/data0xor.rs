#[doc = "Register `DATA0XOR` reader"]
pub type R = crate::R<Data0xorSpec>;
#[doc = "Register `DATA0XOR` writer"]
pub type W = crate::W<Data0xorSpec>;
#[doc = "Field `DATA0XOR` reader - XOR Data 0 Access"]
pub type Data0xorR = crate::FieldReader<u32>;
#[doc = "Field `DATA0XOR` writer - XOR Data 0 Access"]
pub type Data0xorW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - XOR Data 0 Access"]
    #[inline(always)]
    pub fn data0xor(&self) -> Data0xorR {
        Data0xorR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - XOR Data 0 Access"]
    #[inline(always)]
    pub fn data0xor(&mut self) -> Data0xorW<'_, Data0xorSpec> {
        Data0xorW::new(self, 0)
    }
}
#[doc = "DATA0XOR Register Access\n\nYou can [`read`](crate::Reg::read) this register and get [`data0xor::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data0xor::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\n<div class=\"warning\">One or more dependent resources other than the current register are immediately affected by a read operation.</div>"]
pub struct Data0xorSpec;
impl crate::RegisterSpec for Data0xorSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data0xor::R`](R) reader structure"]
impl crate::Readable for Data0xorSpec {}
#[doc = "`write(|w| ..)` method takes [`data0xor::W`](W) writer structure"]
impl crate::Writable for Data0xorSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DATA0XOR to value 0"]
impl crate::Resettable for Data0xorSpec {}
