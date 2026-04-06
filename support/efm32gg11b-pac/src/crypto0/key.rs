#[doc = "Register `KEY` reader"]
pub type R = crate::R<KeySpec>;
#[doc = "Register `KEY` writer"]
pub type W = crate::W<KeySpec>;
#[doc = "Field `KEY` reader - Key Access"]
pub type KeyR = crate::FieldReader<u32>;
#[doc = "Field `KEY` writer - Key Access"]
pub type KeyW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Key Access"]
    #[inline(always)]
    pub fn key(&self) -> KeyR {
        KeyR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Key Access"]
    #[inline(always)]
    pub fn key(&mut self) -> KeyW<'_, KeySpec> {
        KeyW::new(self, 0)
    }
}
#[doc = "KEY Register Access\n\nYou can [`read`](crate::Reg::read) this register and get [`key::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\n<div class=\"warning\">One or more dependent resources other than the current register are immediately affected by a read operation.</div>"]
pub struct KeySpec;
impl crate::RegisterSpec for KeySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`key::R`](R) reader structure"]
impl crate::Readable for KeySpec {}
#[doc = "`write(|w| ..)` method takes [`key::W`](W) writer structure"]
impl crate::Writable for KeySpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets KEY to value 0"]
impl crate::Resettable for KeySpec {}
