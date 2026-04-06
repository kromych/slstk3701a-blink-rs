#[doc = "Register `IFC` writer"]
pub type W = crate::W<IfcSpec>;
#[doc = "Field `VMONAVDDFALL` writer - Clear VMONAVDDFALL Interrupt Flag"]
pub type VmonavddfallW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VMONAVDDRISE` writer - Clear VMONAVDDRISE Interrupt Flag"]
pub type VmonavddriseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VMONALTAVDDFALL` writer - Clear VMONALTAVDDFALL Interrupt Flag"]
pub type VmonaltavddfallW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VMONALTAVDDRISE` writer - Clear VMONALTAVDDRISE Interrupt Flag"]
pub type VmonaltavddriseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VMONDVDDFALL` writer - Clear VMONDVDDFALL Interrupt Flag"]
pub type VmondvddfallW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VMONDVDDRISE` writer - Clear VMONDVDDRISE Interrupt Flag"]
pub type VmondvddriseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VMONIO0FALL` writer - Clear VMONIO0FALL Interrupt Flag"]
pub type Vmonio0fallW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VMONIO0RISE` writer - Clear VMONIO0RISE Interrupt Flag"]
pub type Vmonio0riseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VMONIO1FALL` writer - Clear VMONIO1FALL Interrupt Flag"]
pub type Vmonio1fallW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VMONIO1RISE` writer - Clear VMONIO1RISE Interrupt Flag"]
pub type Vmonio1riseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `R5VREADY` writer - Clear R5VREADY Interrupt Flag"]
pub type R5vreadyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VMONBUVDDFALL` writer - Clear VMONBUVDDFALL Interrupt Flag"]
pub type VmonbuvddfallW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VMONBUVDDRISE` writer - Clear VMONBUVDDRISE Interrupt Flag"]
pub type VmonbuvddriseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PFETOVERCURRENTLIMIT` writer - Clear PFETOVERCURRENTLIMIT Interrupt Flag"]
pub type PfetovercurrentlimitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NFETOVERCURRENTLIMIT` writer - Clear NFETOVERCURRENTLIMIT Interrupt Flag"]
pub type NfetovercurrentlimitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCDCLPRUNNING` writer - Clear DCDCLPRUNNING Interrupt Flag"]
pub type DcdclprunningW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCDCLNRUNNING` writer - Clear DCDCLNRUNNING Interrupt Flag"]
pub type DcdclnrunningW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCDCINBYPASS` writer - Clear DCDCINBYPASS Interrupt Flag"]
pub type DcdcinbypassW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BURDY` writer - Clear BURDY Interrupt Flag"]
pub type BurdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `R5VVSINT` writer - Clear R5VVSINT Interrupt Flag"]
pub type R5vvsintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EM23WAKEUP` writer - Clear EM23WAKEUP Interrupt Flag"]
pub type Em23wakeupW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VSCALEDONE` writer - Clear VSCALEDONE Interrupt Flag"]
pub type VscaledoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEMP` writer - Clear TEMP Interrupt Flag"]
pub type TempW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEMPLOW` writer - Clear TEMPLOW Interrupt Flag"]
pub type TemplowW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEMPHIGH` writer - Clear TEMPHIGH Interrupt Flag"]
pub type TemphighW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Clear VMONAVDDFALL Interrupt Flag"]
    #[inline(always)]
    pub fn vmonavddfall(&mut self) -> VmonavddfallW<'_, IfcSpec> {
        VmonavddfallW::new(self, 0)
    }
    #[doc = "Bit 1 - Clear VMONAVDDRISE Interrupt Flag"]
    #[inline(always)]
    pub fn vmonavddrise(&mut self) -> VmonavddriseW<'_, IfcSpec> {
        VmonavddriseW::new(self, 1)
    }
    #[doc = "Bit 2 - Clear VMONALTAVDDFALL Interrupt Flag"]
    #[inline(always)]
    pub fn vmonaltavddfall(&mut self) -> VmonaltavddfallW<'_, IfcSpec> {
        VmonaltavddfallW::new(self, 2)
    }
    #[doc = "Bit 3 - Clear VMONALTAVDDRISE Interrupt Flag"]
    #[inline(always)]
    pub fn vmonaltavddrise(&mut self) -> VmonaltavddriseW<'_, IfcSpec> {
        VmonaltavddriseW::new(self, 3)
    }
    #[doc = "Bit 4 - Clear VMONDVDDFALL Interrupt Flag"]
    #[inline(always)]
    pub fn vmondvddfall(&mut self) -> VmondvddfallW<'_, IfcSpec> {
        VmondvddfallW::new(self, 4)
    }
    #[doc = "Bit 5 - Clear VMONDVDDRISE Interrupt Flag"]
    #[inline(always)]
    pub fn vmondvddrise(&mut self) -> VmondvddriseW<'_, IfcSpec> {
        VmondvddriseW::new(self, 5)
    }
    #[doc = "Bit 6 - Clear VMONIO0FALL Interrupt Flag"]
    #[inline(always)]
    pub fn vmonio0fall(&mut self) -> Vmonio0fallW<'_, IfcSpec> {
        Vmonio0fallW::new(self, 6)
    }
    #[doc = "Bit 7 - Clear VMONIO0RISE Interrupt Flag"]
    #[inline(always)]
    pub fn vmonio0rise(&mut self) -> Vmonio0riseW<'_, IfcSpec> {
        Vmonio0riseW::new(self, 7)
    }
    #[doc = "Bit 8 - Clear VMONIO1FALL Interrupt Flag"]
    #[inline(always)]
    pub fn vmonio1fall(&mut self) -> Vmonio1fallW<'_, IfcSpec> {
        Vmonio1fallW::new(self, 8)
    }
    #[doc = "Bit 9 - Clear VMONIO1RISE Interrupt Flag"]
    #[inline(always)]
    pub fn vmonio1rise(&mut self) -> Vmonio1riseW<'_, IfcSpec> {
        Vmonio1riseW::new(self, 9)
    }
    #[doc = "Bit 10 - Clear R5VREADY Interrupt Flag"]
    #[inline(always)]
    pub fn r5vready(&mut self) -> R5vreadyW<'_, IfcSpec> {
        R5vreadyW::new(self, 10)
    }
    #[doc = "Bit 12 - Clear VMONBUVDDFALL Interrupt Flag"]
    #[inline(always)]
    pub fn vmonbuvddfall(&mut self) -> VmonbuvddfallW<'_, IfcSpec> {
        VmonbuvddfallW::new(self, 12)
    }
    #[doc = "Bit 13 - Clear VMONBUVDDRISE Interrupt Flag"]
    #[inline(always)]
    pub fn vmonbuvddrise(&mut self) -> VmonbuvddriseW<'_, IfcSpec> {
        VmonbuvddriseW::new(self, 13)
    }
    #[doc = "Bit 16 - Clear PFETOVERCURRENTLIMIT Interrupt Flag"]
    #[inline(always)]
    pub fn pfetovercurrentlimit(&mut self) -> PfetovercurrentlimitW<'_, IfcSpec> {
        PfetovercurrentlimitW::new(self, 16)
    }
    #[doc = "Bit 17 - Clear NFETOVERCURRENTLIMIT Interrupt Flag"]
    #[inline(always)]
    pub fn nfetovercurrentlimit(&mut self) -> NfetovercurrentlimitW<'_, IfcSpec> {
        NfetovercurrentlimitW::new(self, 17)
    }
    #[doc = "Bit 18 - Clear DCDCLPRUNNING Interrupt Flag"]
    #[inline(always)]
    pub fn dcdclprunning(&mut self) -> DcdclprunningW<'_, IfcSpec> {
        DcdclprunningW::new(self, 18)
    }
    #[doc = "Bit 19 - Clear DCDCLNRUNNING Interrupt Flag"]
    #[inline(always)]
    pub fn dcdclnrunning(&mut self) -> DcdclnrunningW<'_, IfcSpec> {
        DcdclnrunningW::new(self, 19)
    }
    #[doc = "Bit 20 - Clear DCDCINBYPASS Interrupt Flag"]
    #[inline(always)]
    pub fn dcdcinbypass(&mut self) -> DcdcinbypassW<'_, IfcSpec> {
        DcdcinbypassW::new(self, 20)
    }
    #[doc = "Bit 22 - Clear BURDY Interrupt Flag"]
    #[inline(always)]
    pub fn burdy(&mut self) -> BurdyW<'_, IfcSpec> {
        BurdyW::new(self, 22)
    }
    #[doc = "Bit 23 - Clear R5VVSINT Interrupt Flag"]
    #[inline(always)]
    pub fn r5vvsint(&mut self) -> R5vvsintW<'_, IfcSpec> {
        R5vvsintW::new(self, 23)
    }
    #[doc = "Bit 24 - Clear EM23WAKEUP Interrupt Flag"]
    #[inline(always)]
    pub fn em23wakeup(&mut self) -> Em23wakeupW<'_, IfcSpec> {
        Em23wakeupW::new(self, 24)
    }
    #[doc = "Bit 25 - Clear VSCALEDONE Interrupt Flag"]
    #[inline(always)]
    pub fn vscaledone(&mut self) -> VscaledoneW<'_, IfcSpec> {
        VscaledoneW::new(self, 25)
    }
    #[doc = "Bit 29 - Clear TEMP Interrupt Flag"]
    #[inline(always)]
    pub fn temp(&mut self) -> TempW<'_, IfcSpec> {
        TempW::new(self, 29)
    }
    #[doc = "Bit 30 - Clear TEMPLOW Interrupt Flag"]
    #[inline(always)]
    pub fn templow(&mut self) -> TemplowW<'_, IfcSpec> {
        TemplowW::new(self, 30)
    }
    #[doc = "Bit 31 - Clear TEMPHIGH Interrupt Flag"]
    #[inline(always)]
    pub fn temphigh(&mut self) -> TemphighW<'_, IfcSpec> {
        TemphighW::new(self, 31)
    }
}
#[doc = "Interrupt Flag Clear Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifc::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IfcSpec;
impl crate::RegisterSpec for IfcSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ifc::W`](W) writer structure"]
impl crate::Writable for IfcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IFC to value 0"]
impl crate::Resettable for IfcSpec {}
