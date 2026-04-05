#[doc = "Register `DEVINSTRRDCONFIG` reader"]
pub type R = crate::R<DevinstrrdconfigSpec>;
#[doc = "Register `DEVINSTRRDCONFIG` writer"]
pub type W = crate::W<DevinstrrdconfigSpec>;
#[doc = "Field `RDOPCODENONXIP` reader - Read Opcode in Non-XIP Mode"]
pub type RdopcodenonxipR = crate::FieldReader;
#[doc = "Field `RDOPCODENONXIP` writer - Read Opcode in Non-XIP Mode"]
pub type RdopcodenonxipW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INSTRTYPE` reader - Instruction Type"]
pub type InstrtypeR = crate::FieldReader;
#[doc = "Field `INSTRTYPE` writer - Instruction Type"]
pub type InstrtypeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DDREN` reader - DDR Enable"]
pub type DdrenR = crate::BitReader;
#[doc = "Field `DDREN` writer - DDR Enable"]
pub type DdrenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRXFERTYPESTDMODE` reader - Address Transfer Type for Standard SPI Modes"]
pub type AddrxfertypestdmodeR = crate::FieldReader;
#[doc = "Field `ADDRXFERTYPESTDMODE` writer - Address Transfer Type for Standard SPI Modes"]
pub type AddrxfertypestdmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DATAXFERTYPEEXTMODE` reader - Data Transfer Type for Standard SPI Modes"]
pub type DataxfertypeextmodeR = crate::FieldReader;
#[doc = "Field `DATAXFERTYPEEXTMODE` writer - Data Transfer Type for Standard SPI Modes"]
pub type DataxfertypeextmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MODEBITENABLE` reader - Mode Bit Enable"]
pub type ModebitenableR = crate::BitReader;
#[doc = "Field `MODEBITENABLE` writer - Mode Bit Enable"]
pub type ModebitenableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DUMMYRDCLKCYCLES` reader - Dummy Read Clock Cycles"]
pub type DummyrdclkcyclesR = crate::FieldReader;
#[doc = "Field `DUMMYRDCLKCYCLES` writer - Dummy Read Clock Cycles"]
pub type DummyrdclkcyclesW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:7 - Read Opcode in Non-XIP Mode"]
    #[inline(always)]
    pub fn rdopcodenonxip(&self) -> RdopcodenonxipR {
        RdopcodenonxipR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:9 - Instruction Type"]
    #[inline(always)]
    pub fn instrtype(&self) -> InstrtypeR {
        InstrtypeR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - DDR Enable"]
    #[inline(always)]
    pub fn ddren(&self) -> DdrenR {
        DdrenR::new(((self.bits >> 10) & 1) != 0)
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
    #[doc = "Bit 20 - Mode Bit Enable"]
    #[inline(always)]
    pub fn modebitenable(&self) -> ModebitenableR {
        ModebitenableR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 24:28 - Dummy Read Clock Cycles"]
    #[inline(always)]
    pub fn dummyrdclkcycles(&self) -> DummyrdclkcyclesR {
        DummyrdclkcyclesR::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Read Opcode in Non-XIP Mode"]
    #[inline(always)]
    pub fn rdopcodenonxip(&mut self) -> RdopcodenonxipW<'_, DevinstrrdconfigSpec> {
        RdopcodenonxipW::new(self, 0)
    }
    #[doc = "Bits 8:9 - Instruction Type"]
    #[inline(always)]
    pub fn instrtype(&mut self) -> InstrtypeW<'_, DevinstrrdconfigSpec> {
        InstrtypeW::new(self, 8)
    }
    #[doc = "Bit 10 - DDR Enable"]
    #[inline(always)]
    pub fn ddren(&mut self) -> DdrenW<'_, DevinstrrdconfigSpec> {
        DdrenW::new(self, 10)
    }
    #[doc = "Bits 12:13 - Address Transfer Type for Standard SPI Modes"]
    #[inline(always)]
    pub fn addrxfertypestdmode(&mut self) -> AddrxfertypestdmodeW<'_, DevinstrrdconfigSpec> {
        AddrxfertypestdmodeW::new(self, 12)
    }
    #[doc = "Bits 16:17 - Data Transfer Type for Standard SPI Modes"]
    #[inline(always)]
    pub fn dataxfertypeextmode(&mut self) -> DataxfertypeextmodeW<'_, DevinstrrdconfigSpec> {
        DataxfertypeextmodeW::new(self, 16)
    }
    #[doc = "Bit 20 - Mode Bit Enable"]
    #[inline(always)]
    pub fn modebitenable(&mut self) -> ModebitenableW<'_, DevinstrrdconfigSpec> {
        ModebitenableW::new(self, 20)
    }
    #[doc = "Bits 24:28 - Dummy Read Clock Cycles"]
    #[inline(always)]
    pub fn dummyrdclkcycles(&mut self) -> DummyrdclkcyclesW<'_, DevinstrrdconfigSpec> {
        DummyrdclkcyclesW::new(self, 24)
    }
}
#[doc = "Device Read Instruction Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`devinstrrdconfig::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devinstrrdconfig::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevinstrrdconfigSpec;
impl crate::RegisterSpec for DevinstrrdconfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`devinstrrdconfig::R`](R) reader structure"]
impl crate::Readable for DevinstrrdconfigSpec {}
#[doc = "`write(|w| ..)` method takes [`devinstrrdconfig::W`](W) writer structure"]
impl crate::Writable for DevinstrrdconfigSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DEVINSTRRDCONFIG to value 0x03"]
impl crate::Resettable for DevinstrrdconfigSpec {
    const RESET_VALUE: u32 = 0x03;
}
