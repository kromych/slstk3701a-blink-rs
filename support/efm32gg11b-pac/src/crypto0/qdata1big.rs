#[doc = "Register `QDATA1BIG` reader"]
pub type R = crate::R<Qdata1bigSpec>;
#[doc = "Register `QDATA1BIG` writer"]
pub type W = crate::W<Qdata1bigSpec>;
#[doc = "Field `QDATA1BIG` reader - Quad Data 1 Big Endian Access"]
pub type Qdata1bigR = crate::FieldReader<u32>;
#[doc = "Field `QDATA1BIG` writer - Quad Data 1 Big Endian Access"]
pub type Qdata1bigW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Quad Data 1 Big Endian Access"]
    #[inline(always)]
    pub fn qdata1big(&self) -> Qdata1bigR {
        Qdata1bigR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Quad Data 1 Big Endian Access"]
    #[inline(always)]
    pub fn qdata1big(&mut self) -> Qdata1bigW<'_, Qdata1bigSpec> {
        Qdata1bigW::new(self, 0)
    }
}
#[doc = "QDATA1 Register Big Endian Access\n\nYou can [`read`](crate::Reg::read) this register and get [`qdata1big::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qdata1big::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\n<div class=\"warning\">One or more dependent resources other than the current register are immediately affected by a read operation.</div>"]
pub struct Qdata1bigSpec;
impl crate::RegisterSpec for Qdata1bigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`qdata1big::R`](R) reader structure"]
impl crate::Readable for Qdata1bigSpec {}
#[doc = "`write(|w| ..)` method takes [`qdata1big::W`](W) writer structure"]
impl crate::Writable for Qdata1bigSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets QDATA1BIG to value 0"]
impl crate::Resettable for Qdata1bigSpec {}
