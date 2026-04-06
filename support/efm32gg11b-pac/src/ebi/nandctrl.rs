#[doc = "Register `NANDCTRL` reader"]
pub type R = crate::R<NandctrlSpec>;
#[doc = "Register `NANDCTRL` writer"]
pub type W = crate::W<NandctrlSpec>;
#[doc = "Field `EN` reader - NAND Flash Control Enable"]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - NAND Flash Control Enable"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "NAND Flash Bank\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Banksel {
    #[doc = "0: Memory bank 0 is connected to a NAND Flash device."]
    Bank0 = 0,
    #[doc = "1: Memory bank 1 is connected to a NAND Flash device."]
    Bank1 = 1,
    #[doc = "2: Memory bank 2 is connected to a NAND Flash device."]
    Bank2 = 2,
    #[doc = "3: Memory bank 3 is connected to a NAND Flash device."]
    Bank3 = 3,
}
impl From<Banksel> for u8 {
    #[inline(always)]
    fn from(variant: Banksel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Banksel {
    type Ux = u8;
}
impl crate::IsEnum for Banksel {}
#[doc = "Field `BANKSEL` reader - NAND Flash Bank"]
pub type BankselR = crate::FieldReader<Banksel>;
impl BankselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Banksel {
        match self.bits {
            0 => Banksel::Bank0,
            1 => Banksel::Bank1,
            2 => Banksel::Bank2,
            3 => Banksel::Bank3,
            _ => unreachable!(),
        }
    }
    #[doc = "Memory bank 0 is connected to a NAND Flash device."]
    #[inline(always)]
    pub fn is_bank0(&self) -> bool {
        *self == Banksel::Bank0
    }
    #[doc = "Memory bank 1 is connected to a NAND Flash device."]
    #[inline(always)]
    pub fn is_bank1(&self) -> bool {
        *self == Banksel::Bank1
    }
    #[doc = "Memory bank 2 is connected to a NAND Flash device."]
    #[inline(always)]
    pub fn is_bank2(&self) -> bool {
        *self == Banksel::Bank2
    }
    #[doc = "Memory bank 3 is connected to a NAND Flash device."]
    #[inline(always)]
    pub fn is_bank3(&self) -> bool {
        *self == Banksel::Bank3
    }
}
#[doc = "Field `BANKSEL` writer - NAND Flash Bank"]
pub type BankselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Banksel, crate::Safe>;
impl<'a, REG> BankselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Memory bank 0 is connected to a NAND Flash device."]
    #[inline(always)]
    pub fn bank0(self) -> &'a mut crate::W<REG> {
        self.variant(Banksel::Bank0)
    }
    #[doc = "Memory bank 1 is connected to a NAND Flash device."]
    #[inline(always)]
    pub fn bank1(self) -> &'a mut crate::W<REG> {
        self.variant(Banksel::Bank1)
    }
    #[doc = "Memory bank 2 is connected to a NAND Flash device."]
    #[inline(always)]
    pub fn bank2(self) -> &'a mut crate::W<REG> {
        self.variant(Banksel::Bank2)
    }
    #[doc = "Memory bank 3 is connected to a NAND Flash device."]
    #[inline(always)]
    pub fn bank3(self) -> &'a mut crate::W<REG> {
        self.variant(Banksel::Bank3)
    }
}
impl R {
    #[doc = "Bit 0 - NAND Flash Control Enable"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:5 - NAND Flash Bank"]
    #[inline(always)]
    pub fn banksel(&self) -> BankselR {
        BankselR::new(((self.bits >> 4) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - NAND Flash Control Enable"]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<'_, NandctrlSpec> {
        EnW::new(self, 0)
    }
    #[doc = "Bits 4:5 - NAND Flash Bank"]
    #[inline(always)]
    pub fn banksel(&mut self) -> BankselW<'_, NandctrlSpec> {
        BankselW::new(self, 4)
    }
}
#[doc = "NAND Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`nandctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nandctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NandctrlSpec;
impl crate::RegisterSpec for NandctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nandctrl::R`](R) reader structure"]
impl crate::Readable for NandctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`nandctrl::W`](W) writer structure"]
impl crate::Writable for NandctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets NANDCTRL to value 0"]
impl crate::Resettable for NandctrlSpec {}
