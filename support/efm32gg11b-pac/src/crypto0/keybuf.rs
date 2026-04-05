#[doc = "Register `KEYBUF` reader"]
pub type R = crate::R<KeybufSpec>;
#[doc = "Register `KEYBUF` writer"]
pub type W = crate::W<KeybufSpec>;
#[doc = "Field `KEYBUF` reader - Key Buffer Access"]
pub type KeybufR = crate::FieldReader<u32>;
#[doc = "Field `KEYBUF` writer - Key Buffer Access"]
pub type KeybufW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Key Buffer Access"]
    #[inline(always)]
    pub fn keybuf(&self) -> KeybufR {
        KeybufR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Key Buffer Access"]
    #[inline(always)]
    pub fn keybuf(&mut self) -> KeybufW<'_, KeybufSpec> {
        KeybufW::new(self, 0)
    }
}
#[doc = "KEY Buffer Register Access\n\nYou can [`read`](crate::Reg::read) this register and get [`keybuf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keybuf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\n<div class=\"warning\">One or more dependent resources other than the current register are immediately affected by a read operation.</div>"]
pub struct KeybufSpec;
impl crate::RegisterSpec for KeybufSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`keybuf::R`](R) reader structure"]
impl crate::Readable for KeybufSpec {}
#[doc = "`write(|w| ..)` method takes [`keybuf::W`](W) writer structure"]
impl crate::Writable for KeybufSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets KEYBUF to value 0"]
impl crate::Resettable for KeybufSpec {}
