#[doc = "Register `DIEPEMPMSK` reader"]
pub type R = crate::R<DiepempmskSpec>;
#[doc = "Register `DIEPEMPMSK` writer"]
pub type W = crate::W<DiepempmskSpec>;
#[doc = "Field `INEPTXFEMPMSK` reader - IN EP Tx FIFO Empty Interrupt Mask Bits"]
pub type IneptxfempmskR = crate::FieldReader<u16>;
#[doc = "Field `INEPTXFEMPMSK` writer - IN EP Tx FIFO Empty Interrupt Mask Bits"]
pub type IneptxfempmskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - IN EP Tx FIFO Empty Interrupt Mask Bits"]
    #[inline(always)]
    pub fn ineptxfempmsk(&self) -> IneptxfempmskR {
        IneptxfempmskR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - IN EP Tx FIFO Empty Interrupt Mask Bits"]
    #[inline(always)]
    pub fn ineptxfempmsk(&mut self) -> IneptxfempmskW<'_, DiepempmskSpec> {
        IneptxfempmskW::new(self, 0)
    }
}
#[doc = "Device IN Endpoint FIFO Empty Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`diepempmsk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepempmsk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DiepempmskSpec;
impl crate::RegisterSpec for DiepempmskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`diepempmsk::R`](R) reader structure"]
impl crate::Readable for DiepempmskSpec {}
#[doc = "`write(|w| ..)` method takes [`diepempmsk::W`](W) writer structure"]
impl crate::Writable for DiepempmskSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DIEPEMPMSK to value 0"]
impl crate::Resettable for DiepempmskSpec {}
