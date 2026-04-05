#[doc = "Register `MIR1_DATAL` reader"]
pub type R = crate::R<Mir1DatalSpec>;
#[doc = "Register `MIR1_DATAL` writer"]
pub type W = crate::W<Mir1DatalSpec>;
#[doc = "Field `DATA0` reader - First Byte of CAN Data Frame"]
pub type Data0R = crate::FieldReader;
#[doc = "Field `DATA0` writer - First Byte of CAN Data Frame"]
pub type Data0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DATA1` reader - Second Byte of CAN Data Frame"]
pub type Data1R = crate::FieldReader;
#[doc = "Field `DATA1` writer - Second Byte of CAN Data Frame"]
pub type Data1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DATA2` reader - Third Byte of CAN Data Frame"]
pub type Data2R = crate::FieldReader;
#[doc = "Field `DATA2` writer - Third Byte of CAN Data Frame"]
pub type Data2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DATA3` reader - Fourth Byte of CAN Data Frame"]
pub type Data3R = crate::FieldReader;
#[doc = "Field `DATA3` writer - Fourth Byte of CAN Data Frame"]
pub type Data3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - First Byte of CAN Data Frame"]
    #[inline(always)]
    pub fn data0(&self) -> Data0R {
        Data0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Second Byte of CAN Data Frame"]
    #[inline(always)]
    pub fn data1(&self) -> Data1R {
        Data1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Third Byte of CAN Data Frame"]
    #[inline(always)]
    pub fn data2(&self) -> Data2R {
        Data2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Fourth Byte of CAN Data Frame"]
    #[inline(always)]
    pub fn data3(&self) -> Data3R {
        Data3R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - First Byte of CAN Data Frame"]
    #[inline(always)]
    pub fn data0(&mut self) -> Data0W<'_, Mir1DatalSpec> {
        Data0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Second Byte of CAN Data Frame"]
    #[inline(always)]
    pub fn data1(&mut self) -> Data1W<'_, Mir1DatalSpec> {
        Data1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Third Byte of CAN Data Frame"]
    #[inline(always)]
    pub fn data2(&mut self) -> Data2W<'_, Mir1DatalSpec> {
        Data2W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Fourth Byte of CAN Data Frame"]
    #[inline(always)]
    pub fn data3(&mut self) -> Data3W<'_, Mir1DatalSpec> {
        Data3W::new(self, 24)
    }
}
#[doc = "Interface Data a Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mir1_datal::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mir1_datal::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mir1DatalSpec;
impl crate::RegisterSpec for Mir1DatalSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mir1_datal::R`](R) reader structure"]
impl crate::Readable for Mir1DatalSpec {}
#[doc = "`write(|w| ..)` method takes [`mir1_datal::W`](W) writer structure"]
impl crate::Writable for Mir1DatalSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MIR1_DATAL to value 0"]
impl crate::Resettable for Mir1DatalSpec {}
