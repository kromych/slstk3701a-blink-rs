#[doc = "Register `IF1IEN` reader"]
pub type R = crate::R<If1ienSpec>;
#[doc = "Register `IF1IEN` writer"]
pub type W = crate::W<If1ienSpec>;
#[doc = "Field `STATUS` reader - STATUS Interrupt Enable"]
pub type StatusR = crate::BitReader;
#[doc = "Field `STATUS` writer - STATUS Interrupt Enable"]
pub type StatusW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - STATUS Interrupt Enable"]
    #[inline(always)]
    pub fn status(&self) -> StatusR {
        StatusR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - STATUS Interrupt Enable"]
    #[inline(always)]
    pub fn status(&mut self) -> StatusW<'_, If1ienSpec> {
        StatusW::new(self, 0)
    }
}
#[doc = "Status Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`if1ien::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`if1ien::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct If1ienSpec;
impl crate::RegisterSpec for If1ienSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`if1ien::R`](R) reader structure"]
impl crate::Readable for If1ienSpec {}
#[doc = "`write(|w| ..)` method takes [`if1ien::W`](W) writer structure"]
impl crate::Writable for If1ienSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IF1IEN to value 0x01"]
impl crate::Resettable for If1ienSpec {
    const RESET_VALUE: u32 = 0x01;
}
