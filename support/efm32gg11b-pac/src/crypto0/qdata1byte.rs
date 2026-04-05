#[doc = "Register `QDATA1BYTE` reader"]
pub type R = crate::R<Qdata1byteSpec>;
#[doc = "Register `QDATA1BYTE` writer"]
pub type W = crate::W<Qdata1byteSpec>;
#[doc = "Field `QDATA1BYTE` reader - Qdata 1 Byte Access"]
pub type Qdata1byteR = crate::FieldReader;
#[doc = "Field `QDATA1BYTE` writer - Qdata 1 Byte Access"]
pub type Qdata1byteW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Qdata 1 Byte Access"]
    #[inline(always)]
    pub fn qdata1byte(&self) -> Qdata1byteR {
        Qdata1byteR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Qdata 1 Byte Access"]
    #[inline(always)]
    pub fn qdata1byte(&mut self) -> Qdata1byteW<'_, Qdata1byteSpec> {
        Qdata1byteW::new(self, 0)
    }
}
#[doc = "QDATA1 Register Byte Access\n\nYou can [`read`](crate::Reg::read) this register and get [`qdata1byte::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qdata1byte::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\n<div class=\"warning\">One or more dependent resources other than the current register are immediately affected by a read operation.</div>"]
pub struct Qdata1byteSpec;
impl crate::RegisterSpec for Qdata1byteSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`qdata1byte::R`](R) reader structure"]
impl crate::Readable for Qdata1byteSpec {}
#[doc = "`write(|w| ..)` method takes [`qdata1byte::W`](W) writer structure"]
impl crate::Writable for Qdata1byteSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets QDATA1BYTE to value 0"]
impl crate::Resettable for Qdata1byteSpec {}
