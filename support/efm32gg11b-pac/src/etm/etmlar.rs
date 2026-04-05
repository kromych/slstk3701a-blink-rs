#[doc = "Register `ETMLAR` reader"]
pub type R = crate::R<EtmlarSpec>;
#[doc = "Register `ETMLAR` writer"]
pub type W = crate::W<EtmlarSpec>;
#[doc = "Field `KEY` reader - Key Value"]
pub type KeyR = crate::BitReader;
#[doc = "Field `KEY` writer - Key Value"]
pub type KeyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Key Value"]
    #[inline(always)]
    pub fn key(&self) -> KeyR {
        KeyR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Key Value"]
    #[inline(always)]
    pub fn key(&mut self) -> KeyW<'_, EtmlarSpec> {
        KeyW::new(self, 0)
    }
}
#[doc = "ETM Lock Access Register\n\nYou can [`read`](crate::Reg::read) this register and get [`etmlar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`etmlar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EtmlarSpec;
impl crate::RegisterSpec for EtmlarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`etmlar::R`](R) reader structure"]
impl crate::Readable for EtmlarSpec {}
#[doc = "`write(|w| ..)` method takes [`etmlar::W`](W) writer structure"]
impl crate::Writable for EtmlarSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ETMLAR to value 0"]
impl crate::Resettable for EtmlarSpec {}
