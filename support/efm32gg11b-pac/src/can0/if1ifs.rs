#[doc = "Register `IF1IFS` writer"]
pub type W = crate::W<If1ifsSpec>;
#[doc = "Field `STATUS` writer - Set STATUS Interrupt Flag"]
pub type StatusW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Set STATUS Interrupt Flag"]
    #[inline(always)]
    pub fn status(&mut self) -> StatusW<'_, If1ifsSpec> {
        StatusW::new(self, 0)
    }
}
#[doc = "Message Object Interrupt Flag Set Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`if1ifs::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct If1ifsSpec;
impl crate::RegisterSpec for If1ifsSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`if1ifs::W`](W) writer structure"]
impl crate::Writable for If1ifsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IF1IFS to value 0"]
impl crate::Resettable for If1ifsSpec {}
