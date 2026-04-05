#[doc = "Register `DEVINSTRWRCONFIG` reader"]
pub type R = crate::R<DevinstrwrconfigSpec>;
#[doc = "Register `DEVINSTRWRCONFIG` writer"]
pub type W = crate::W<DevinstrwrconfigSpec>;
#[doc = "Field `WROPCODE` reader - Write Opcode"]
pub type WropcodeR = crate::FieldReader;
#[doc = "Field `WROPCODE` writer - Write Opcode"]
pub type WropcodeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `WELDIS` reader - WEL Disable"]
pub type WeldisR = crate::BitReader;
#[doc = "Field `WELDIS` writer - WEL Disable"]
pub type WeldisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRXFERTYPESTDMODE` reader - Address Transfer Type for Standard SPI Modes"]
pub type AddrxfertypestdmodeR = crate::FieldReader;
#[doc = "Field `ADDRXFERTYPESTDMODE` writer - Address Transfer Type for Standard SPI Modes"]
pub type AddrxfertypestdmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DATAXFERTYPEEXTMODE` reader - Data Transfer Type for Standard SPI Modes"]
pub type DataxfertypeextmodeR = crate::FieldReader;
#[doc = "Field `DATAXFERTYPEEXTMODE` writer - Data Transfer Type for Standard SPI Modes"]
pub type DataxfertypeextmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DUMMYWRCLKCYCLES` reader - Dummy Write Clock Cycles"]
pub type DummywrclkcyclesR = crate::FieldReader;
#[doc = "Field `DUMMYWRCLKCYCLES` writer - Dummy Write Clock Cycles"]
pub type DummywrclkcyclesW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:7 - Write Opcode"]
    #[inline(always)]
    pub fn wropcode(&self) -> WropcodeR {
        WropcodeR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - WEL Disable"]
    #[inline(always)]
    pub fn weldis(&self) -> WeldisR {
        WeldisR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Address Transfer Type for Standard SPI Modes"]
    #[inline(always)]
    pub fn addrxfertypestdmode(&self) -> AddrxfertypestdmodeR {
        AddrxfertypestdmodeR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Data Transfer Type for Standard SPI Modes"]
    #[inline(always)]
    pub fn dataxfertypeextmode(&self) -> DataxfertypeextmodeR {
        DataxfertypeextmodeR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 24:28 - Dummy Write Clock Cycles"]
    #[inline(always)]
    pub fn dummywrclkcycles(&self) -> DummywrclkcyclesR {
        DummywrclkcyclesR::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Write Opcode"]
    #[inline(always)]
    pub fn wropcode(&mut self) -> WropcodeW<'_, DevinstrwrconfigSpec> {
        WropcodeW::new(self, 0)
    }
    #[doc = "Bit 8 - WEL Disable"]
    #[inline(always)]
    pub fn weldis(&mut self) -> WeldisW<'_, DevinstrwrconfigSpec> {
        WeldisW::new(self, 8)
    }
    #[doc = "Bits 12:13 - Address Transfer Type for Standard SPI Modes"]
    #[inline(always)]
    pub fn addrxfertypestdmode(&mut self) -> AddrxfertypestdmodeW<'_, DevinstrwrconfigSpec> {
        AddrxfertypestdmodeW::new(self, 12)
    }
    #[doc = "Bits 16:17 - Data Transfer Type for Standard SPI Modes"]
    #[inline(always)]
    pub fn dataxfertypeextmode(&mut self) -> DataxfertypeextmodeW<'_, DevinstrwrconfigSpec> {
        DataxfertypeextmodeW::new(self, 16)
    }
    #[doc = "Bits 24:28 - Dummy Write Clock Cycles"]
    #[inline(always)]
    pub fn dummywrclkcycles(&mut self) -> DummywrclkcyclesW<'_, DevinstrwrconfigSpec> {
        DummywrclkcyclesW::new(self, 24)
    }
}
#[doc = "Device Write Instruction Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`devinstrwrconfig::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devinstrwrconfig::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevinstrwrconfigSpec;
impl crate::RegisterSpec for DevinstrwrconfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`devinstrwrconfig::R`](R) reader structure"]
impl crate::Readable for DevinstrwrconfigSpec {}
#[doc = "`write(|w| ..)` method takes [`devinstrwrconfig::W`](W) writer structure"]
impl crate::Writable for DevinstrwrconfigSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DEVINSTRWRCONFIG to value 0x02"]
impl crate::Resettable for DevinstrwrconfigSpec {
    const RESET_VALUE: u32 = 0x02;
}
