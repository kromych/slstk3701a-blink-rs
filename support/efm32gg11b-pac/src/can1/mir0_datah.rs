#[doc = "Register `MIR0_DATAH` reader"]
pub type R = crate::R<Mir0DatahSpec>;
#[doc = "Register `MIR0_DATAH` writer"]
pub type W = crate::W<Mir0DatahSpec>;
#[doc = "Field `DATA4` reader - Fifth Byte of CAN Data Frame"]
pub type Data4R = crate::FieldReader;
#[doc = "Field `DATA4` writer - Fifth Byte of CAN Data Frame"]
pub type Data4W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DATA5` reader - Sixth Byte of CAN Data Frame"]
pub type Data5R = crate::FieldReader;
#[doc = "Field `DATA5` writer - Sixth Byte of CAN Data Frame"]
pub type Data5W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DATA6` reader - Seventh Byte of CAN Data Frame"]
pub type Data6R = crate::FieldReader;
#[doc = "Field `DATA6` writer - Seventh Byte of CAN Data Frame"]
pub type Data6W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DATA7` reader - Eight Byte of CAN Data Frame"]
pub type Data7R = crate::FieldReader;
#[doc = "Field `DATA7` writer - Eight Byte of CAN Data Frame"]
pub type Data7W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Fifth Byte of CAN Data Frame"]
    #[inline(always)]
    pub fn data4(&self) -> Data4R {
        Data4R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Sixth Byte of CAN Data Frame"]
    #[inline(always)]
    pub fn data5(&self) -> Data5R {
        Data5R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Seventh Byte of CAN Data Frame"]
    #[inline(always)]
    pub fn data6(&self) -> Data6R {
        Data6R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Eight Byte of CAN Data Frame"]
    #[inline(always)]
    pub fn data7(&self) -> Data7R {
        Data7R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Fifth Byte of CAN Data Frame"]
    #[inline(always)]
    pub fn data4(&mut self) -> Data4W<'_, Mir0DatahSpec> {
        Data4W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Sixth Byte of CAN Data Frame"]
    #[inline(always)]
    pub fn data5(&mut self) -> Data5W<'_, Mir0DatahSpec> {
        Data5W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Seventh Byte of CAN Data Frame"]
    #[inline(always)]
    pub fn data6(&mut self) -> Data6W<'_, Mir0DatahSpec> {
        Data6W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Eight Byte of CAN Data Frame"]
    #[inline(always)]
    pub fn data7(&mut self) -> Data7W<'_, Mir0DatahSpec> {
        Data7W::new(self, 24)
    }
}
#[doc = "Interface Data B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mir0_datah::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mir0_datah::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mir0DatahSpec;
impl crate::RegisterSpec for Mir0DatahSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mir0_datah::R`](R) reader structure"]
impl crate::Readable for Mir0DatahSpec {}
#[doc = "`write(|w| ..)` method takes [`mir0_datah::W`](W) writer structure"]
impl crate::Writable for Mir0DatahSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MIR0_DATAH to value 0"]
impl crate::Resettable for Mir0DatahSpec {}
