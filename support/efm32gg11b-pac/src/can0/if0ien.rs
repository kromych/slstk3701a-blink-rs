#[doc = "Register `IF0IEN` reader"]
pub type R = crate::R<If0ienSpec>;
#[doc = "Register `IF0IEN` writer"]
pub type W = crate::W<If0ienSpec>;
#[doc = "Field `MESSAGE` reader - MESSAGE Interrupt Enable"]
pub type MessageR = crate::FieldReader<u32>;
#[doc = "Field `MESSAGE` writer - MESSAGE Interrupt Enable"]
pub type MessageW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - MESSAGE Interrupt Enable"]
    #[inline(always)]
    pub fn message(&self) -> MessageR {
        MessageR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - MESSAGE Interrupt Enable"]
    #[inline(always)]
    pub fn message(&mut self) -> MessageW<'_, If0ienSpec> {
        MessageW::new(self, 0)
    }
}
#[doc = "Message Object Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`if0ien::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`if0ien::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct If0ienSpec;
impl crate::RegisterSpec for If0ienSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`if0ien::R`](R) reader structure"]
impl crate::Readable for If0ienSpec {}
#[doc = "`write(|w| ..)` method takes [`if0ien::W`](W) writer structure"]
impl crate::Writable for If0ienSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IF0IEN to value 0xffff_ffff"]
impl crate::Resettable for If0ienSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
