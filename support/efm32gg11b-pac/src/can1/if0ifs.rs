#[doc = "Register `IF0IFS` writer"]
pub type W = crate::W<If0ifsSpec>;
#[doc = "Field `MESSAGE` writer - Set MESSAGE Interrupt Flag"]
pub type MessageW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Set MESSAGE Interrupt Flag"]
    #[inline(always)]
    pub fn message(&mut self) -> MessageW<'_, If0ifsSpec> {
        MessageW::new(self, 0)
    }
}
#[doc = "Message Object Interrupt Flag Set Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`if0ifs::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct If0ifsSpec;
impl crate::RegisterSpec for If0ifsSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`if0ifs::W`](W) writer structure"]
impl crate::Writable for If0ifsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IF0IFS to value 0"]
impl crate::Resettable for If0ifsSpec {}
